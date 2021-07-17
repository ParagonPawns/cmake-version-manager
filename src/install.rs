pub fn install_version(args: &Vec<String>, cvm_home: &str) {
    if args.len() > 3 {
        log_error("Option recieve more than expected arguments");
        return
    }

    let releases = match cached_releases(cvm_home) {
        Ok(releases) => releases,
        Err(error) => {
            log_error(
                &format!("Failed to retrieve cached releases. ({})", error)
            );
            return
        }
    };

    let tag = if args.len() == 2 {
        if releases.is_empty() {
            println!("Seem that we do not have any cached CMake releases. Try \
                cleaning with 'cvm remove --all' and try again");
            return
        }

        let message = String::from("Please select a cmake verson to install:");
        let mut builder = IList::<String>::new(message);

        for i in 0..releases.len() {
            if i == 11 {
                break
            }

            builder = builder.add_item(&releases[i], releases[i].clone());
        }

        let result = builder.inquire();

        match result {
            Ok(selected) => selected,
            Err(inq_msg) => match inq_msg {
                InquiryMessage::CloseRequested => {
                    println!("\nSession was canceled. Exiting...");
                    return
                },
                _ => {
                    log_error("Inquiry failed. exiting session");
                    return
                }
            }
        }
    } else {
        args[2].clone()
    };

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

    if is_installed(&tag, &installed_versions) {
        println!("Version {} is already installed. Switching...", tag);
        switch(&tag, cvm_home);
        return
    }

    if !download(cvm_home, &tag) {
        return
    }

    set_current_install(cvm_home, &tag);
    set_installed(cvm_home, &tag);

}

struct HelperStrings {
    bins_path: String,
    download_url: String,
    save_path: String,
    server_name: String,
}


#[cfg(target_os="macos")]
fn pre_19_2(cvm_home: &str, version: &str) -> HelperStrings {
    let name = format!("cmake-{}", version);
    let server_name = format!("{}-Darwin-x86_64", name);
    let download_url = format!(
        "https://github.com/Kitware/CMake/releases/download/v{}/{}.tar.gz",
        version,
        server_name
    );

    let bins_path = format!("{}/bins", cvm_home);
    let save_path = format!("{}/{}.tar.gz", bins_path, name);

    HelperStrings {
        bins_path,
        download_url,
        save_path,
        server_name,
    }
}

#[cfg(target_os="macos")]
fn post_19_1(cvm_home: &str, version: &str) -> HelperStrings {
    let name = format!("cmake-{}", version);

    let system = System::new();
    let os_version = match system.get_os_version() {
        Some(version) => version,
        None => panic!("Failed to get the os version")
    };

    let mac_version = parse_version(&os_version);

    let dw_name = if mac_version.major < 10 || mac_version.major == 10 && mac_version.minor <= 10 {
        "macos10.10-universal"
    } else {
        "macos-universal"
    };

    let server_name = format!("{}-{}", name, dw_name);
    let download_url = format!(
        "https://github.com/Kitware/CMake/releases/download/v{}/{}.tar.gz",
        version,
        server_name
    );

    let bins_path = format!("{}/bins", cvm_home);
    let save_path = format!("{}/{}.tar.gz", bins_path, name);

    HelperStrings {
        bins_path,
        download_url,
        save_path,
        server_name,
    }
}

#[cfg(target_os="macos")]
fn download_strings(cvm_home: &str, version: &str) -> HelperStrings {
    let cmake_version = parse_version(version);
    if cmake_version.minor < 19 || cmake_version.minor == 19 && cmake_version.patch <= 1 {
        return pre_19_2(cvm_home, version)
    }

    post_19_1(cvm_home, version)
}

#[allow(dead_code)]
struct Version {
    major: i32,
    minor: i32,
    patch: i32,
}

fn parse_version(version: &str) -> utils::Version {
    let version_clean = match version.find('-') {
        Some(index) => &version[0..index],
        None => version
    };

    utils::parse_version(version_clean)
}

#[cfg(target_os="linux")] #[cfg(target_arch="x86_64")]
fn download_strings(cvm_home: &str, version: &str) -> HelperStrings {
    let cmake_version = parse_version(version);

    let linux = if cmake_version.minor > 19 { "linux" } else { "Linux" };
    let name = format!("cmake-{}", version);
    let server_name = format!("{}-{}-x86_64", name, linux);
    let download_url = format!(
        "https://github.com/Kitware/CMake/releases/download/v{}/{}.tar.gz",
        version,
        server_name
    );

    let bins_path = format!("{}/bins", cvm_home);
    let save_path = format!("{}/{}.tar.gz", bins_path, name);

    HelperStrings {
        bins_path,
        download_url,
        save_path,
        server_name,
    }
}

#[cfg(windows)]
fn post_19(cvm_home: &str, version: &str) -> HelperStrings {
    let arch = if cfg!(target_arch="x86") {
        "windows-i386"
    } else {
        "windows-x86_64"
    };

    let name = format!("cmake-{}", version);
    let server_name = format!("{}-{}", name, arch);
    let download_url = format!(
        "https://github.com/Kitware/CMake/releases/download/v{}/{}.zip",
        version,
        server_name
    );

    let bins_path = format!("{}/bins", cvm_home);
    let save_path = format!("{}/{}.zip", bins_path, name);

    HelperStrings {
        bins_path,
        download_url,
        save_path,
        server_name,
    }
}

#[cfg(windows)]
fn pre_20(cvm_home: &str, version: &str) -> HelperStrings {
    let arch = if cfg!(target_arch="x86") {
        "32-x86"
    } else {
        "64-x64"
    };

    let name = format!("cmake-{}", version);
    let server_name = format!("{}-win{}", name, arch);
    let download_url = format!(
        "https://github.com/Kitware/CMake/releases/download/v{}/{}.zip",
        version,
        server_name
    );

    let bins_path = format!("{}/bins", cvm_home);
    let save_path = format!("{}/{}.zip", bins_path, name);

    HelperStrings {
        bins_path,
        download_url,
        save_path,
        server_name,
    }
}

#[cfg(windows)]
fn download_strings(cvm_home: &str, version: &str) -> HelperStrings {
    let cmake_version = parse_version(version);
    if cmake_version.minor >= 20 {
        return post_19(cvm_home, version)
    }

    pre_20(cvm_home, version)
}

fn download(cvm_home: &str, version: &str) -> bool {
    let strings = download_strings(cvm_home, version);
    let mut easy = Easy2::new(Collector(Vec::new()));

    if let Err(error) = easy.url(&strings.download_url) {
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
                &format!("Specified version: {}, failed to install or doesnt exist. Response code: {}", version, code)
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
    let file_result = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&strings.save_path);

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
    println!("{} - {}", strings.save_path, strings.bins_path);

    let command = format!(
        "tar -xf \"{}\" -C \"{}\"",
        strings.save_path,
        strings.bins_path
    );

    #[cfg(unix)]
    let program = "sh";

    #[cfg(windows)]
    let program = "powershell";

    let result = std::process::Command::new(program)
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

    let from = format!("{}/{}", strings.bins_path, strings.server_name);
    let to = format!("{}/current", strings.bins_path);

    if let Err(error) = std::fs::rename(from, to) {
        log_error(&format!("Failed to rename directory. ({})", error));
        return false
    }

    if let Err(error) = std::fs::remove_file(strings.save_path) {
        log_error(&format!("Failed to cleanup download. ({})", error));
        return false
    }

    true
}

#[cfg(target_os="macos")]
use sysinfo::{ System, SystemExt };

use std::fs::OpenOptions;
use std::io::Write;
use std::process::Stdio;

use curl::easy::{ Easy2, List };

use term_inquiry::{ List as IList, InquiryMessage };

use crate::switch::switch;
use crate::releases::{
    cached_releases,
    currently_installed,
    set_current_install,
    set_installed,
    installed,
    is_installed
};
use crate::Collector;
use crate::log::log_error;
use crate::utils;