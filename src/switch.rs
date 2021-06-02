pub fn switch_version(args: &Vec<String>, cvm_home: &str) {
    let installed = match installed(cvm_home) {
        Ok(installed) => installed,
        Err(error) => {
            log_error(&format!("Failed to retrieve installed versions. ({})", error));
            return
        }
    };

    let tag = if args.len() == 2 {
        if installed.is_empty() {
            println!("There are no installed versions of CMake that we are tracking.");
            return
        }

        let message = String::from("Please select a cmake verson to switch to:");
        let mut builder = List::<String>::new(message);

        for i in 0.. installed.len() {
            builder = builder.add_item(&installed[i], installed[i].clone());
        }

        let result = builder.render();

        match result {
            Ok(selected) => selected,
            Err(inq_msg) => match inq_msg {
                InqueryMessage::CloseRequested => {
                    println!("\nSession was canceled. Exiting...");
                    return
                },
                _ => {
                    log_error("Inquiry failed. exiting session");
                    return
                }
            }
        }
    } else {
        args[2].clone()
    };

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

    println!("Checking to see if version is already installed...");
    if !is_installed(&tag, &installed) {
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

    println!("Switching...");
    switch(&tag, cvm_home);
    set_current_install(cvm_home, &tag);
    println!("Successfully switch to CMake v{}", tag);
}

pub fn switch(version: &str, cvm_home:&str) {
    let from = format!("{}/bins/cmake-{}", cvm_home, version);
    let to = format!("{}/bins/current", cvm_home);

    if let Err(error) = std::fs::rename(from, to) {
        log_error(&format!("Failed to rename directory. ({})", error));
        return
    }
}

use term_inquiry::{ List, InqueryMessage };

use crate::releases::{
    currently_installed,
    installed,
    is_installed,
    set_current_install
};
use crate::log::log_error;
