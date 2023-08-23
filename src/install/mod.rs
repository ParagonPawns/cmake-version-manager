mod helper_strings;

pub fn install_version(args: &Vec<Rc<str>>, cvm_home: &Path) -> Result<(), Rc<str>> {
    if args.len() > 3 {
        return Err(TOO_MANY_ARGS_STR.into());
    }

    let releases = cached_releases(cvm_home)?;
    let tag = get_tag(&releases, args)?;

    if tag.is_empty() {
        return Ok(());
    }

    // We don't mind if no versions are currently installed.
    let current = current_version(cvm_home).unwrap_or(Rc::from(""));
    if *tag == *current {
        println!("CMake v{}, has already been installed and selected.", tag);
        return Ok(());
    }

    if !current.is_empty() {
        let from = cvm_home.join(crate::CVM_BINS).join(crate::CVM_CURRENT_DIR);
        let to = cvm_home
            .join(crate::CVM_BINS)
            .join(format!("cmake-{}", current));

        std::fs::rename(from, to).map_err(map_error!("Failed to rename directory. ({})"))?;
    }

    let installed_versions = installed(cvm_home)?;

    if is_installed(&tag, &installed_versions) {
        println!("Version {} is already installed. Switching...", tag);
        switch(&tag, cvm_home)?;
        return Ok(());
    }

    download(cvm_home, &tag)?;
    set_current_install(cvm_home, &tag)?;
    set_installed(cvm_home, &tag)?;

    Ok(())
}

fn get_tag(releases: &Vec<Rc<str>>, args: &Vec<Rc<str>>) -> Result<Rc<str>, Rc<str>> {
    if args.len() == 3 {
        return Ok(args[2].clone());
    }

    if releases.is_empty() {
        return Err(NO_RELEASES_FOUND_STR.into());
    }

    let mut builder = IList::<Rc<str>>::new("Please select a cmake verson to install:");

    for i in 0..releases.len() {
        if i == 11 {
            break;
        }

        builder = builder.add_item(releases[i].as_ref(), releases[i].clone());
    }

    let result = builder.inquire();

    match result {
        Ok(selected) => Ok(Rc::from(selected)),
        Err(inq_msg) => match inq_msg {
            InquiryMessage::CloseRequested => {
                println!("\nSession was canceled. Exiting...");
                Ok("".into())
            }
            _ => Err("Inquiry failed. exiting session".into()),
        },
    }
}

fn download(cvm_home: &Path, version: &str) -> Result<(), Rc<str>> {
    let strings = helper_strings::HelperStrings::new(cvm_home, version)?;

    println!("Downloading CMake {}...", version);
    let byte_data = blocking::Client::new()
        .get(strings.download_url)
        .header(
            header::USER_AGENT,
            format!("cvm {} request", env!("CARGO_PKG_VERSION")),
        )
        .header(header::ACCEPT, "application/vnd.github.v3+json")
        .send()
        .map_err(map_error!("Failed to request releases from github. ({})"))?
        .bytes()
        .map_err(map_error!("Failed to get bytes from request. ({})"))?;

    println!("Writing data to file...");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&strings.save_path)
        .map_err(map_error!("Faild to create save file for download. ({})"))?;

    file.write_all(byte_data.as_ref())
        .map_err(map_error!("Failed to writed saved data to file. ({})"))?;

    println!("Extracting...");
    println!(
        "{} - {}",
        strings.save_path.to_str().unwrap_or(""),
        strings.bins_path.to_str().unwrap_or("")
    );

    let command = format!(
        r#"tar -xf "{}" -C "{}""#,
        strings.save_path.to_str().unwrap_or(""),
        strings.bins_path.to_str().unwrap_or("")
    );

    #[cfg(unix)]
    let program = "sh";

    #[cfg(windows)]
    let program = "powershell";

    let mut child = std::process::Command::new(program)
        .arg("-c")
        .arg(&command)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(map_error!("Failed to spawn unzip command. ({})"))?;

    child
        .wait()
        .map_err(map_error!("Failed to run unzip command. ({})"))?;

    let from = strings.bins_path.join(strings.server_name);
    let to = strings.bins_path.join(crate::CVM_CURRENT_DIR);

    std::fs::rename(from, to).map_err(map_error!("Failed to rename directory. ({})"))?;

    std::fs::remove_file(strings.save_path)
        .map_err(map_error!("Failed to cleanup download. ({})"))?;

    Ok(())
}

const NO_RELEASES_FOUND_STR: &'static str = "Seems that we do not have any cached CMake releases.\nTry cleaning with 'cvm remove --all' and try again";
const TOO_MANY_ARGS_STR: &'static str =
    "Command 'install' must only contain version to install or be empty for interactive.";

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::Stdio;
use std::rc::Rc;

use reqwest::{blocking, header};
use term_inquiry::{InquiryMessage, List as IList};

use crate::macros::map_error;
use crate::releases::{
    cached_releases, current_version, installed, is_installed, set_current_install, set_installed,
};
use crate::switch::switch;
