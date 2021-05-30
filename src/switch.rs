pub fn switch_version(args: &Vec<String>, cvm_home: &str) {
    if args.len() == 2 {
        // todo interactive
        return
    }

    let tag = &args[2];
    let current = match currently_installed(cvm_home) {
        Ok(current) => current,
        Err(error) => {
            log_error(&format!("Failed to get currently installed version. ({})", error));
            return
        }
    };

    if *tag == current {
        println!("CMake version {} is already selected.", tag);
        return
    }

    let installed = match installed(cvm_home) {
        Ok(installed) => installed,
        Err(error) => {
            log_error(&format!("Failed to retrieve installed versions. ({})", error));
            return
        }
    };

    if !is_installed(tag, &installed) {
        println!("CMake version {0} is not installed. Please run 'cvm install {0}'", tag);
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

    switch(tag, cvm_home);
    set_current_install(cvm_home, tag);
}

pub fn switch(version: &str, cvm_home:&str) {
    let from = format!("{}/bins/cmake-{}", cvm_home, version);
    let to = format!("{}/bins/current", cvm_home);

    if let Err(error) = std::fs::rename(from, to) {
        log_error(&format!("Failed to rename directory. ({})", error));
        return
    }
}

use crate::releases::{
    currently_installed,
    installed,
    is_installed,
    set_current_install
};
use crate::log::log_error;
