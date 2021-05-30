pub fn install_version(args: &Vec<String>, cvm_home: &str) {
    if args.len() == 2 {
        // todo list choice
        return
    }

    if args.len() > 3 {
        log_error("Option recieve more than expected arguments");
        return
    }

    let tag = &args[2];
    let current = match currently_installed(cvm_home) {
        Ok(version) => version,
        Err(error) => {
            log_error(&format!("Failed to get currently install version. ({})", error));
            String::new()
        }
    };

    if *tag == current {
        println!("CMake v{}, has already been installed and selected.", tag);
        return
    }

    if !current.is_empty() {
        let from = format!("{}/bins/current", cvm_home);
        let to = format!("{}/bins/cmake-{}", cvm_home, current);

        if let Err(error) = std::fs::rename(from, to) {
            log_error(&format!("Failed to rename directory. ({})", error));
            return
        }
    }

    let installed_versions = match installed(cvm_home) {
        Ok(versions) => versions,
        Err(error) => {
            log_error(&format!("Failed to retrieve installed versions. ({})", error));
            return
        }
    };

    if is_installed(tag, &installed_versions) {
        println!("Version {} is already installed. Switching...", tag);
        switch(tag, cvm_home);
        return
    }

    if !download(cvm_home, args) {
        return
    }

    set_current_install(cvm_home, &args[2]);
    set_installed(cvm_home, &args[2]);

}

#[cfg(target_os="linux")] #[cfg(target_arch="x86_64")]
fn download(cvm_home: &str, args: &Vec<String>) -> bool {
    let mut easy = Easy2::new(Collector(Vec::new()));

    let url = format!(
        "https://github.com/Kitware/CMake/releases/download/v{0}/cmake-{0}-linux-x86_64.tar.gz",
        args[2]
    );

    if let Err(error) = easy.url(&url) {
        log_error(
            &format!("Failed to set url for fetch cmake releases. ({})", error)
        );
        return false
    }

    if let Err(error) = easy.get(true) {
        log_error(
            &format!("Failed to set request method as GET. ({})", error)
        );
        return false
    }

    if let Err(error) = easy.follow_location(true) {
        log_error(&format!("Failed to set url redirect. ({})", error));
        return false
    }

    let mut headers = List::new();
    if let Err(error) = headers.append("Accept: application/vnd.github.v3+json") {
        log_error(
            &format!("Failed to append header item to list. ({})", error)
        );
        return false
    }

    if let Err(error) = headers.append("User-Agent: request") {
        log_error(
            &format!("Failed to append header item to list. ({})", error)
        );
        return false
    }

    if let Err(error) = easy.http_headers(headers) {
        log_error(
            &format!("Failed to set http headers. ({})", error)
        );
        return false
    }

    println!("CMake download started...");
    if let Err(error) = easy.perform() {
        log_error(
            &format!("Failed at executing request. ({})", error)
        );
        return false
    }

    match easy.response_code() {
        Ok(code) => if code != 200 {
            log_error(
                &format!("Specified version: {}, failed to install or doesnt exist. Response code: {}", args[2], code)
            );
            return false
        },
        Err(error) => {
            log_error(
                &format!("Failed to retrieve response code. ({})", error)
            );
            return false
        }
    }

    println!("Writing data to file...");
    let bins_path = format!("{}/bins", cvm_home);
    let name = format!("cmake-{}", args[2]);
    let save_path = format!("{}/{}.tar.gz", bins_path, name);
    let file_result = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&save_path);

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => {
            log_error(&format!("Faild to create save file for download. ({})", error));
            return false
        }
    };

    if let Err(error) = file.write_all(easy.get_ref().0.as_slice()) {
        log_error(&format!("Failed to writed saved data to file. ({})", error));
        return false
    }

    println!("Extracting...");
    let command = format!("tar -xf {} -C {}", save_path, bins_path);
    let result = std::process::Command::new("sh")
        .arg("-c")
        .arg(&command)
        .stdout(Stdio::piped())
        .spawn();

    let status_result = match result {
        Ok(mut child) => child.wait(),
        Err(error) => {
            log_error(&format!("Failed to spawn unzip command. ({})", error));
            return false
        }
    };

    if let Err(error) = status_result {
        log_error(&format!("Failed to run unzip command. ({})", error));
        return false
    }

    let from = format!("{}/{}-linux-x86_64", bins_path, name);
    let to = format!("{}/current", bins_path);

    if let Err(error) = std::fs::rename(from, to) {
        log_error(&format!("Failed to rename directory. ({})", error));
        return false
    }

    if let Err(error) = std::fs::remove_file(save_path) {
        log_error(&format!("Failed to cleanup download. ({})", error));
        return false
    }

    true
}

use std::fs::OpenOptions;
use std::io::Write;
use std::process::Stdio;

use curl::easy::{ Easy2, List };

use crate::switch::switch;
use crate::releases::{
    currently_installed,
    set_current_install,
    set_installed,
    installed,
    is_installed
};
use crate::Collector;
use crate::log::log_error;
