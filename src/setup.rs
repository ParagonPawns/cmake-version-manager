fn validate_caches(cvm_home: &str) {
    let result = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(String::from(cvm_home) + crate::CVM_CACHE);

    let mut reader = match result {
        Ok(file) => std::io::BufReader::new(file),
        Err(error) => {
            log_error(&format!("Failed to open cvm_cache file. ({})", error));
            return
        }
    };

    let latest = latest_release();
    let latest_release = &latest[1..];
    let mut line = String::new();

    if let Err(error) = reader.read_line(&mut line) {
        log_error(&format!("Failed to read first line in cache file. ({})", error));
        return
    }

    line.pop();

    // check if cache is updated.
    if line == latest_release {
        return
    }

    println!("New release detected updating available versions...");
    let releases = releases();
    let mut file = reader.into_inner();

    if let Err(error) = file.set_len(0) {
        log_error(&format!("Failed to clear cache file. ({})", error));
        return
    }

    if let Err(error) = file.seek(std::io::SeekFrom::Start(0)) {
        log_error(&format!("Failed to seek to beginning of file. ({})", error));
        return
    }

    for mut release in releases {
        release.push('\n');
        if let Err(error) = file.write(release[1..].as_bytes()) {
            log_error(
                &format!("Failed to write release to cache file. ({})", error)
            );
        }
    }
}

pub fn setup_cvm(cvm_home: &str) -> bool {
    if Path::new(cvm_home).exists() {
        validate_caches(cvm_home);
        return true
    }

    println!("'.cvm' directory is not set up. Setting up now...");
    // create our directory
    match fs::create_dir(cvm_home) {
        Ok(..) => {},
        Err(error) => {
            log_error(&format!("Failed to create directory. ({})", error));
            return false
        }
    };

    match fs::create_dir(String::from(cvm_home) + "/bins") {
        Ok(..) => {},
        Err(error) => {
            log_error(&format!("Failed to create bins directory. ({})", error));
            return false
        }
    };

    match fs::File::create(String::from(cvm_home) + crate::CVM_CACHE) {
        Ok(..) => {},
        Err(error) => {
            log_error(&format!("Failed to create cvm_cache file. ({})", error));
            return false
        }
    }

    match fs::File::create(String::from(cvm_home) + crate::CVM_INSTALLED) {
        Ok(..) => {},
        Err(error) => {
            log_error(&format!("Failed to create cvm_installed file. ({})", error));
            return false
        }
    }

    match fs::File::create(String::from(cvm_home) + crate::CVM_CURRENT) {
        Ok(..) => {},
        Err(error) => {
            log_error(&format!("Failed to create cvm_current file. ({})", error));
            return false
        }
    }

    validate_caches(cvm_home);
    true
}

use std::io::Seek;
use std::path::Path;
use std::fs;
use std::io::{ BufRead, Write };

use crate::log::log_error;
use crate::releases::{ latest_release, releases };
