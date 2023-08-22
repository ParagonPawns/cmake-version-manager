#[derive(Deserialize)]
struct Release {
    tag_name: Rc<str>,
}

pub fn releases() -> Result<Vec<Rc<str>>, Rc<str>> {
    let response = blocking::Client::new()
        .get("https://api.github.com/repos/Kitware/CMake/releases?per_page=100")
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
        })?;

    let releases = response
        .json::<Vec<Release>>()
        .map_err(|error| {
            Rc::from(format!(
                "Failed to parse releases from response. ({})",
                error
            ))
        })?
        .iter()
        .map(|release| release.tag_name[1..].into()) // remove prefix 'v' from the version
        .collect();

    Ok(releases)
}

pub fn cached_releases(cvm_home: &Path) -> Result<Vec<Rc<str>>, Rc<str>> {
    let file_path = cvm_home.join(crate::CVM_CACHE);
    let file = std::fs::File::open(file_path)
        .map_err(|error| Rc::from(format!("Failed to get cached releases. ({})", error)))?;
    let buf = std::io::BufReader::new(file).lines();

    let mut cached_versions = Vec::new();
    for line in buf {
        if let Ok(version) = line {
            cached_versions.push(Rc::from(version));
        }
    }

    Ok(cached_versions)
}

pub fn installed(cvm_home: &Path) -> Result<Vec<Rc<str>>, Rc<str>> {
    const ERR_STR: &'static str = "Failed to get installed cmake versions.";
    let file_path = cvm_home.join(crate::CVM_INSTALLED);
    let file = std::fs::File::open(file_path)
        .map_err(|error| Rc::from(format!("{} ({})", ERR_STR, error)))?;

    let buf = std::io::BufReader::new(file).lines();
    let mut installed_versions = Vec::new();

    for line in buf {
        if let Ok(version) = line {
            installed_versions.push(Rc::from(version));
        }
    }

    Ok(installed_versions)
}

pub fn current_version(cvm_home: &Path) -> Result<Rc<str>, Rc<str>> {
    const ERR_STR: &'static str = "Failed to get currently installed version.";
    let file_path = cvm_home.join(crate::CVM_CURRENT_FILE);
    let file = std::fs::File::open(file_path)
        .map_err(|error| Rc::from(format!("{} ({})", ERR_STR, error)))?;
    let mut lines = std::io::BufReader::new(file).lines();

    match lines.next() {
        Some(current_version) => match current_version {
            Ok(version) => Ok(version.into()),
            Err(error) => Err(format!("{} ({})", ERR_STR, error).into()),
        },
        None => Err(format!("{} (No version of cmake has been installed)", ERR_STR).into()),
    }
}

pub fn is_installed(in_version: &str, installed: &Vec<Rc<str>>) -> bool {
    for version in installed {
        if version.as_ref() == in_version {
            return true;
        }
    }

    false
}

pub fn set_installed(cvm_home: &Path, install: &str) -> Result<(), Rc<str>> {
    let file_path = cvm_home.join(crate::CVM_INSTALLED);
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .map_err(|error| Rc::from(format!("Failed to open cvm_current file. ({})", error)))?;

    let data = format!("{}\n", install);
    file.write(data.as_bytes()).map_err(|error| {
        Rc::from(format!(
            "Failed to write current install to file. ({})",
            error
        ))
    })?;

    Ok(())
}

pub fn set_current_install(cvm_home: &Path, install: &str) -> Result<(), Rc<str>> {
    let file_path = cvm_home.join(crate::CVM_CURRENT_FILE);
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .open(file_path)
        .map_err(|error| Rc::from(format!("Failed to open cvm_current file. ({})", error)))?;

    file.set_len(0)
        .map_err(|error| Rc::from(format!("Failed to clear cvm_current file. ({})", error)))?;

    file.seek(std::io::SeekFrom::Start(0))
        .map_err(|error| Rc::from(format!("Failed to seek to beginning of file. ({})", error)))?;

    file.write(install.as_bytes()).map_err(|error| {
        Rc::from(format!(
            "Failed to write current install to file. ({})",
            error
        ))
    })?;

    Ok(())
}

pub fn latest_release() -> Result<Rc<str>, Rc<str>> {
    let response = blocking::Client::new()
        .get("https://api.github.com/repos/Kitware/CMake/releases/latest")
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
        })?;

    let release = response.json::<Release>().map_err(|error| {
        Rc::from(format!(
            "Failed to parse releases from response. ({})",
            error
        ))
    })?;

    Ok(release.tag_name[1..].into())
}

use std::io::Write;
use std::io::{BufRead, Seek};
use std::path::Path;
use std::rc::Rc;

use reqwest::{blocking, header};
use serde::Deserialize;
