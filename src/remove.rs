pub fn remove(args: &Vec<String>, cvm_home: &str) {
    let mut installed = match installed(cvm_home) {
        Ok(installed) => installed,
        Err(error) => {
            log_error(
                &format!("Failed to retieve installed versions. ({})", error)
            );
            return
        }
    };

    let tag = if args.len() == 2 {
        if installed.is_empty() {
            println!("There are no CMake version that is installed that we track.");
            return
        }

        let message = String::from("Please select a cmake verson to remove:");
        let mut builder = List::<String>::new(message);

        for i in 0.. installed.len() {
            builder = builder.add_item(&installed[i], installed[i].clone());
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
    } else if args[2] == "--all" {
        println!("Removing all CMake version and contents in '{}'.", cvm_home);
        if let Err(error) = std::fs::remove_dir_all(cvm_home) {
            log_error(
                &format!("Failed to remove $HOME/.cvm directory. ({})", error)
            );
        }
        return
    } else {
        args[2].clone()
    };

    println!("Checking if the version we are removing is in use...");
    match currently_installed(cvm_home) {
        Ok(version) => if version == tag {
            println!("Cannot remove this version since its the current version \
                     selected. Switch before removing. Or use --all to remove \
                     everything"
            );
            return
        },
        Err(error) => {
            log_error(
                &format!("There was an error getting currently installed version. ({})", error)
            );
            return
        }
    }

    println!("Checking if the version we are trying to removed is selected...");
    let mut found_index = None;
    for i in 0..installed.len() {
        if installed[i] == tag {
            println!("Version found...");
            found_index = Some(i);
            break
        }
    }

    if let Some(index) = found_index {
        let version = installed.remove(index);

        println!("Removing version installation...");
        let dir_to_rm = format!("{}/bins/cmake-{}", cvm_home, version);
        if let Err(error) = std::fs::remove_dir_all(dir_to_rm.clone()) {
            log_error(
                &format!("Failed to remove '{}'. ({})", dir_to_rm, error)
            );
            return
        }

        let file_path = format!("{}{}", cvm_home, crate::CVM_INSTALLED);
        let mut file = match std::fs::OpenOptions::new()
            .write(true)
            .open(file_path) {
            Ok(file) => file,
            Err(error) => {
                log_error(&format!("Failed to open cvm_current file. ({})", error));
                return
            }
        };

        if let Err(error) = file.set_len(0) {
            log_error(
                &format!("Failed to clear cvm_installed file. ({})", error)
            );
            return
        }

        for mut version in installed {
            version.push('\n');

            if let Err(error) = file.write(version.as_bytes()) {
                log_error(&format!("Failed to write current install to file. ({})", error));
                return
            }
        }

        println!("Successfully removed CMake v{}.", tag);
        return
    }

    println!("Selected CMake version v{} is not installed. Therefore cannot be removed", tag);
}

use std::io::Write;

use term_inquiry::{ List, InquiryMessage };

use crate::log::log_error;
use crate::releases::currently_installed;
use crate::releases::installed;
