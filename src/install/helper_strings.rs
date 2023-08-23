pub struct HelperStrings {
    pub bins_path: PathBuf,
    pub download_url: String,
    pub save_path: PathBuf,
    pub server_name: String,
}

impl HelperStrings {
    pub fn new(cvm_home: &Path, version: &str) -> Result<Self, Rc<str>> {
        download_strings(cvm_home, version)
    }
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

fn parse_version(version: &str) -> Result<utils::Version, Rc<str>> {
    let version_clean = match version.find('-') {
        Some(index) => &version[0..index],
        None => version,
    };

    utils::parse_version(version_clean)
}

#[cfg(target_os = "macos")]
use sysinfo::{System, SystemExt};

use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::utils;
