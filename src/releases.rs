pub fn releases() -> Vec<String> {
    let mut easy = Easy2::new(Collector(Vec::new()));

    let url = "https://api.github.com/repos/Kitware/CMake/releases?per_page=100";

    if let Err(error) = easy.url(url) {
        log_error(
            &format!("Failed to set url for fetch cmake releases. ({})", error)
        );
        return Vec::new()
    }

    if let Err(error) = easy.get(true) {
        log_error(
            &format!("Failed to set request method as GET. ({})", error)
        );
        return Vec::new()
    }

    let mut headers = List::new();
    if let Err(error) = headers.append("Accept: application/vnd.github.v3+json") {
        log_error(
            &format!("Failed to append header item to list. ({})", error)
        );
        return Vec::new()
    }

    if let Err(error) = headers.append("User-Agent: request") {
        log_error(
            &format!("Failed to append header item to list. ({})", error)
        );
        return Vec::new()
    }

    if let Err(error) = easy.http_headers(headers) {
        log_error(
            &format!("Failed to set http headers. ({})", error)
        );
        return Vec::new()
    }

    if let Err(error) = easy.perform() {
        log_error(
            &format!("Failed at executing request. ({})", error)
        );
        return Vec::new()
    }

    match easy.response_code() {
        Ok(code) => if code != 200 { log_error(
            &format!("Response failed with error: {}" ,code)
        )},
        Err(error) => {
            log_error(
                &format!("Failed to retrieve response code. ({})", error)
            );
            return Vec::new()
        }
    }

    let json_data = match json::parse(&String::from_utf8_lossy(&easy.get_ref().0)) {
        Ok(data) => data,
        Err(error) => {
            log_error(
                &format!("Failed to parse json data. ({})", error)
            );
            return Vec::new()
        }
    };

    let mut releases = Vec::new();
    for i in 0..json_data.len() {
        match json_data[i]["tag_name"].as_str() {
            Some(release) => releases.push(String::from(release)),
            None => continue
        };
    }

    releases
}

pub fn cached_releases(cvm_home: &str) -> Result<Vec<String>, std::io::Error> {
    let file_path = String::from(cvm_home) + crate::CVM_CACHE;
    let buf = match std::fs::File::open(file_path) {
        Ok(file) => std::io::BufReader::new(file).lines(),
        Err(error) => return Err(error)
    };

    let mut cached_versions = Vec::new();
    for line in buf {
        if let Ok(version) = line {
            cached_versions.push(version);
        }
    }

    Ok(cached_versions)
}

pub fn installed(cvm_home: &str) -> Result<Vec<String>, std::io::Error> {
    let file_path = String::from(cvm_home) + crate::CVM_INSTALLED;
    let buf = match std::fs::File::open(file_path) {
        Ok(file) => std::io::BufReader::new(file).lines(),
        Err(error) => return Err(error)
    };

    let mut installed_versions = Vec::new();
    for line in buf {
        if let Ok(version) = line {
            installed_versions.push(version);
        }
    }

    Ok(installed_versions)
}

pub fn currently_installed(cvm_home: &str) -> Result<String, std::io::Error> {
    let file_path = String::from(cvm_home) + crate::CVM_CURRENT;
    let mut lines = match std::fs::File::open(file_path) {
        Ok(file) => std::io::BufReader::new(file).lines(),
        Err(error) => return Err(error)
    };

    match lines.next() {
        Some(current_version) => {
            match current_version {
                Ok(version) => Ok(version),
                Err(..) => Ok(String::new())
            }
        },
        None => Ok(String::new())
    }
}

pub fn is_installed(in_version: &str, installed: &Vec<String>) -> bool {
    for version in installed {
        if version == in_version {
            return true
        }
    }

    false
}

pub fn set_installed(cvm_home: &str, install: &str) {
    let file_path = format!("{}{}", cvm_home, crate::CVM_INSTALLED);
    let mut file = match std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path) {
        Ok(file) => file,
        Err(error) => {
            log_error(&format!("Failed to open cvm_current file. ({})", error));
            return
        }
    };

    let data = format!("{}\n", install);
    if let Err(error) = file.write(data.as_bytes()) {
        log_error(&format!("Failed to write current install to file. ({})", error));
        return
    }
}

pub fn set_current_install(cvm_home: &str, install: &str) {
    let file_path = format!("{}{}", cvm_home, crate::CVM_CURRENT);
    let mut file = match std::fs::OpenOptions::new().write(true).open(file_path) {
        Ok(file) => file,
        Err(error) => {
            log_error(&format!("Failed to open cvm_current file. ({})", error));
            return
        }
    };

    if let Err(error) = file.set_len(0) {
        log_error(&format!("Failed to clear cvm_current file. ({})", error));
        return
    }

    if let Err(error) = file.seek(std::io::SeekFrom::Start(0)) {
        log_error(&format!("Failed to seek to beginning of file. ({})", error));
        return
    }

    if let Err(error) = file.write(install.as_bytes()) {
        log_error(&format!("Failed to write current install to file. ({})", error));
        return
    }
}

pub fn latest_release() -> String {
    let mut easy = Easy2::new(Collector(Vec::new()));

    let url = "https://api.github.com/repos/Kitware/CMake/releases/latest";

    if let Err(error) = easy.url(url) {
        log_error(
            &format!("Failed to set url for fetch cmake releases. ({})", error)
        );
        return String::new()
    }

    if let Err(error) = easy.get(true) {
        log_error(
            &format!("Failed to set request method as GET. ({})", error)
        );
        return String::new()
    }

    let mut headers = List::new();
    if let Err(error) = headers.append("Accept: application/vnd.github.v3+json") {
        log_error(
            &format!("Failed to append header item to list. ({})", error)
        );
        return String::new()
    }

    if let Err(error) = headers.append("User-Agent: request") {
        log_error(
            &format!("Failed to append header item to list. ({})", error)
        );
        return String::new()

    }

    if let Err(error) = easy.http_headers(headers) {
        log_error(
            &format!("Failed to set http headers. ({})", error)
        );
        return String::new()
    }

    if let Err(error) = easy.perform() {
        log_error(
            &format!("Failed at executing request. ({})", error)
        );
        return String::new()
    }

    match easy.response_code() {
        Ok(code) => if code != 200 { log_error(
            &format!("Response failed with error: {}" ,code)
        )},
        Err(error) => {
            log_error(
                &format!("Failed to retrieve response code. ({})", error)
            );
            return String::new()
        }
    }

    let str_data = String::from_utf8_lossy(&easy.get_ref().0);
    let json_data = match json::parse(&str_data) {
        Ok(data) => data,
        Err(error) => {
            log_error(
                &format!("Failed to parse json data. ({})", error)
            );
            return String::new()
        }
    };

    match json_data["tag_name"].as_str() {
        Some(version) => String::from(version),
        None => String::new()
    }
}

use std::io::Write;
use std::io::{ BufRead, Seek };

use curl::easy::{Easy2, List };
use json;

use crate::Collector;
use crate::log::log_error;
