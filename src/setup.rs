fn validate_caches(cvm_home: &Path) -> Result<(), Rc<str>> {
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(cvm_home.join(crate::CVM_CACHE))
        .map_err(map_error!("Failed to open cvm_cache file. ({})"))?;

    let mut reader = std::io::BufReader::new(file);
    let latest_release = latest_release()?;
    let mut line = String::new();

    reader
        .read_line(&mut line)
        .map_err(map_error!("Failed to read first line in cache file. ({})"))?;

    line.pop();

    // check if cache is updated.
    if line == *latest_release {
        return Ok(());
    }

    println!("New release detected updating available versions...");
    let releases = releases()?;
    let mut file = reader.into_inner();

    file.set_len(0)
        .map_err(map_error!("Failed to clear cache file. ({})"))?;

    file.seek(std::io::SeekFrom::Start(0))
        .map_err(map_error!("Failed to seek to beginning of file. ({})"))?;

    for release in releases {
        let release = format!("{}\n", release);
        file.write(release.as_bytes())
            .map_err(map_error!("Failed to write release to cache file. ({})"))?;
    }

    Ok(())
}

pub fn setup_cvm(cvm_home: &Path) -> Result<(), Rc<str>> {
    if cvm_home.exists() {
        validate_caches(cvm_home)?;
        return Ok(());
    }

    println!("'.cvm' directory is not set up. Setting up now...");

    log::info("Creating cvm home directory...");
    fs::create_dir(cvm_home).map_err(map_error!("Failed to create directory. ({})"))?;

    log::info("Creating bins directory...");
    fs::create_dir(cvm_home.join(crate::CVM_BINS))
        .map_err(map_error!("Failed to create bins directory. ({})"))?;

    log::info("Creating file to cache available versions...");
    fs::File::create(cvm_home.join(crate::CVM_CACHE))
        .map_err(map_error!("Failed to create cvm_cache file. ({})"))?;

    log::info("Creating file to track installed versions...");
    fs::File::create(cvm_home.join(crate::CVM_INSTALLED))
        .map_err(map_error!("Failed to create cvm_installed file. ({})"))?;

    log::info("Creating file to track currently installed version...");
    fs::File::create(cvm_home.join(crate::CVM_CURRENT_FILE))
        .map_err(map_error!("Failed to create cvm_current file. ({})"))?;

    validate_caches(cvm_home)?;
    Ok(())
}

use std::fs;
use std::io::Seek;
use std::io::{BufRead, Write};
use std::path::Path;
use std::rc::Rc;

use crate::log;
use crate::macros::map_error;
use crate::releases::{latest_release, releases};
