pub fn install_version(args: &Vec<Rc<str>>, cvm_home: &Path) -> Result<(), Rc<str>> {
    if args.len() > 3 {
        return Err(Rc::from(
            "Command 'install' must only contain version to install or be empty for interactive.",
        ));
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

        println!(
            "Or here. {} to {}",
            from.to_str().unwrap(),
            to.to_str().unwrap()
        );
        std::fs::rename(from, to)
            .map_err(|error| Rc::from(format!("Failed to rename directory. ({})", error)))?;
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
        return Err("Seem that we do not have any cached CMake releases.\nTry cleaning with 'cvm remove --all' and try again".into());
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

struct HelperStrings {
    bins_path: PathBuf,
    download_url: String,
    save_path: PathBuf,
    server_name: String,
}

#[cfg(target_os = "macos")]
fn pre_19_2(cvm_home: &Path, version: &str) -> HelperStrings {
    let name = format!("cmake-{}", version);
    let server_name = format!("{}-Darwin-x86_64", name);
    let download_url = format!(
        "https://github.com/Kitware/CMake/releases/download/v{}/{}.tar.gz",
        version, server_name
    );

    let bins_path = cvm_home.join(crate::CVM_BINS);
    let save_path = bins_path.clone().join(format!("{}.tar.gz", name));

    HelperStrings {
        bins_path,
        download_url,
        save_path,
        server_name,
    }
}

#[cfg(target_os = "macos")]
fn post_19_1(cvm_home: &Path, version: &str) -> Result<HelperStrings, Rc<str>> {
    let name = format!("cmake-{}", version);

    let system = System::new();
    let os_version = match system.get_os_version() {
        Some(version) => version,
        None => panic!("Failed to get the os version"),
    };

    let mac_version = parse_version(&os_version)?;

    let dw_name = if mac_version.major < 10 || mac_version.major == 10 && mac_version.minor <= 10 {
        "macos10.10-universal"
    } else {
        "macos-universal"
    };

    let server_name = format!("{}-{}", name, dw_name);
    let download_url = format!(
        "https://github.com/Kitware/CMake/releases/download/v{}/{}.tar.gz",
        version, server_name
    );

    let bins_path = cvm_home.join(crate::CVM_BINS);
    let save_path = bins_path.clone().join(format!("{}.tar.gz", name));

    Ok(HelperStrings {
        bins_path,
        download_url,
        save_path,
        server_name,
    })
}

#[cfg(target_os = "macos")]
fn download_strings(cvm_home: &Path, version: &str) -> Result<HelperStrings, Rc<str>> {
    let cmake_version = parse_version(version)?;
    if cmake_version.minor < 19 || cmake_version.minor == 19 && cmake_version.patch <= 1 {
        return Ok(pre_19_2(cvm_home, version));
    }

    post_19_1(cvm_home, version)
}

#[allow(dead_code)]
struct Version {
    major: i32,
    minor: i32,
    patch: i32,
}

fn parse_version(version: &str) -> Result<utils::Version, Rc<str>> {
    let version_clean = match version.find('-') {
        Some(index) => &version[0..index],
        None => version,
    };

    utils::parse_version(version_clean)
}

#[cfg(target_os = "linux")]
#[cfg(target_arch = "x86_64")]
fn download_strings(cvm_home: &Path, version: &str) -> Result<HelperStrings, Rc<str>> {
    let cmake_version = parse_version(version)?;

    let linux = if cmake_version.minor > 19 {
        "linux"
    } else {
        "Linux"
    };
    let name = format!("cmake-{}", version);
    let server_name = format!("{}-{}-x86_64", name, linux);
    let download_url = format!(
        "https://github.com/Kitware/CMake/releases/download/v{}/{}.tar.gz",
        version, server_name
    );

    let bins_path = cvm_home.join(crate::CVM_BINS);
    let save_path = bins_path.clone().join(format!("{}.tar.gz", name));

    Ok(HelperStrings {
        bins_path,
        download_url,
        save_path,
        server_name,
    })
}

#[cfg(windows)]
fn post_19(cvm_home: &Path, version: &str) -> HelperStrings {
    let arch = if cfg!(target_arch = "x86") {
        "windows-i386"
    } else {
        "windows-x86_64"
    };

    let name = format!("cmake-{}", version);
    let server_name = format!("{}-{}", name, arch);
    let download_url = format!(
        "https://github.com/Kitware/CMake/releases/download/v{}/{}.zip",
        version, server_name
    );

    let bins_path = cvm_home.join(crate::CVM_BINS);
    let save_path = bins_path.clone().join(format!("{}.zip", name));

    HelperStrings {
        bins_path,
        download_url,
        save_path,
        server_name,
    }
}

#[cfg(windows)]
fn pre_20(cvm_home: &Path, version: &str) -> HelperStrings {
    let arch = if cfg!(target_arch = "x86") {
        "32-x86"
    } else {
        "64-x64"
    };

    let name = format!("cmake-{}", version);
    let server_name = format!("{}-win{}", name, arch);
    let download_url = format!(
        "https://github.com/Kitware/CMake/releases/download/v{}/{}.zip",
        version, server_name
    );

    let bins_path = cvm_home.join(crate::CVM_BINS);
    let save_path = bins_path.clone().join(format!("{}.zip", name));

    HelperStrings {
        bins_path,
        download_url,
        save_path,
        server_name,
    }
}

#[cfg(windows)]
fn download_strings(cvm_home: &Path, version: &str) -> Result<HelperStrings, Rc<str>> {
    let cmake_version = parse_version(version)?;
    if cmake_version.minor >= 20 {
        return Ok(post_19(cvm_home, version));
    }

    Ok(pre_20(cvm_home, version))
}

fn download(cvm_home: &Path, version: &str) -> Result<(), Rc<str>> {
    let strings = download_strings(cvm_home, version)?;

    println!("Downloading CMake {}...", version);
    let byte_data = blocking::Client::new()
        .get(strings.download_url)
        .header(
            header::USER_AGENT,
            format!("cvm {} request", env!("CARGO_PKG_VERSION")),
        )
        .header(header::ACCEPT, "application/vnd.github.v3+json")
        .send()
        .map_err(|error| {
            Rc::from(format!(
                "Failed to request releases from github. ({})",
                error
            ))
        })?
        .bytes()
        .map_err(|error| Rc::from(format!("Failed to get bytes from request. ({})", error)))?;

    println!("Writing data to file...");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&strings.save_path)
        .map_err(|error| {
            Rc::from(format!(
                "Faild to create save file for download. ({})",
                error
            ))
        })?;

    file.write_all(byte_data.as_ref())
        .map_err(|error| Rc::from(format!("Failed to writed saved data to file. ({})", error)))?;

    println!("Extracting...");
    println!(
        "{} - {}",
        strings.save_path.to_str().unwrap_or(""),
        strings.bins_path.to_str().unwrap_or("")
    );

    let command = format!(
        "tar -xf \"{}\" -C \"{}\"",
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
        .map_err(|error| Rc::from(format!("Failed to spawn unzip command. ({})", error)))?;

    child
        .wait()
        .map_err(|error| Rc::from(format!("Failed to run unzip command. ({})", error)))?;

    let from = strings.bins_path.join(strings.server_name);
    let to = strings.bins_path.join(crate::CVM_CURRENT_DIR);

    std::fs::rename(from, to)
        .map_err(|error| Rc::from(format!("Failed to rename directory. ({})", error)))?;

    std::fs::remove_file(strings.save_path)
        .map_err(|error| Rc::from(format!("Failed to cleanup download. ({})", error)))?;

    Ok(())
}

#[cfg(target_os = "macos")]
use sysinfo::{System, SystemExt};

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::rc::Rc;

use reqwest::{blocking, header};
use term_inquiry::{InquiryMessage, List as IList};

use crate::releases::{
    cached_releases, current_version, installed, is_installed, set_current_install, set_installed,
};
use crate::switch::switch;
use crate::utils;
