pub fn releases() -> Result<Vec<Rc<str>>, Rc<str>> {
    let mut easy = Easy2::new(Collector(Vec::new()));

    let url = "https://api.github.com/repos/Kitware/CMake/releases?per_page=100";

    easy.url(url).map_err(|error| {
        Rc::from(format!(
            "Failed to set url for fetch cmake releases. ({})",
            error
        ))
    })?;

    easy.get(true)
        .map_err(|error| Rc::from(format!("Failed to set request method as GET. ({})", error)))?;

    let mut headers = List::new();
    headers
        .append("Accept: application/vnd.github.v3+json")
        .map_err(|error| Rc::from(format!("Failed to append header item to list. ({})", error)))?;

    headers
        .append("User-Agent: request")
        .map_err(|error| Rc::from(format!("Failed to append header item to list. ({})", error)))?;

    easy.http_headers(headers)
        .map_err(|error| Rc::from(format!("Failed to set http headers. ({})", error)))?;

    easy.perform()
        .map_err(|error| Rc::from(format!("Failed at executing request. ({})", error)))?;

    let response_code = easy
        .response_code()
        .map_err(|error| Rc::from(format!("Failed to retrieve response code. ({})", error)))?;

    if response_code != 200 {
        return Err(format!("Response failed with error: {}", response_code).into());
    }

    let json_data = json::parse(&String::from_utf8_lossy(&easy.get_ref().0))
        .map_err(|error| Rc::from(format!("Failed to parse json data. ({})", error)))?;

    let mut releases = Vec::new();
    for i in 0..json_data.len() {
        match json_data[i]["tag_name"].as_str() {
            Some(release) => releases.push(release[1..].into()),
            None => continue,
        };
    }

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

pub fn latest_release() -> String {
    let mut easy = Easy2::new(Collector(Vec::new()));

    let url = "https://api.github.com/repos/Kitware/CMake/releases/latest";

    if let Err(error) = easy.url(url) {
        log_error(&format!(
            "Failed to set url for fetch cmake releases. ({})",
            error
        ));
        return String::new();
    }

    if let Err(error) = easy.get(true) {
        log_error(&format!("Failed to set request method as GET. ({})", error));
        return String::new();
    }

    let mut headers = List::new();
    if let Err(error) = headers.append("Accept: application/vnd.github.v3+json") {
        log_error(&format!(
            "Failed to append header item to list. ({})",
            error
        ));
        return String::new();
    }

    if let Err(error) = headers.append("User-Agent: request") {
        log_error(&format!(
            "Failed to append header item to list. ({})",
            error
        ));
        return String::new();
    }

    if let Err(error) = easy.http_headers(headers) {
        log_error(&format!("Failed to set http headers. ({})", error));
        return String::new();
    }

    if let Err(error) = easy.perform() {
        log_error(&format!("Failed at executing request. ({})", error));
        return String::new();
    }

    match easy.response_code() {
        Ok(code) => {
            if code != 200 {
                log_error(&format!("Response failed with error: {}", code))
            }
        }
        Err(error) => {
            log_error(&format!("Failed to retrieve response code. ({})", error));
            return String::new();
        }
    }

    let str_data = String::from_utf8_lossy(&easy.get_ref().0);
    let json_data = match json::parse(&str_data) {
        Ok(data) => data,
        Err(error) => {
            log_error(&format!("Failed to parse json data. ({})", error));
            return String::new();
        }
    };

    match json_data["tag_name"].as_str() {
        Some(version) => String::from(version),
        None => String::new(),
    }
}

use std::io::Write;
use std::io::{BufRead, Seek};
use std::path::Path;
use std::rc::Rc;

use curl::easy::{Easy2, List};
use json;

use crate::log::log_error;
use crate::Collector;
