pub fn remove(args: &Vec<Rc<str>>, cvm_home: &Path) -> Result<(), Rc<str>> {
    let mut installed = installed(cvm_home)?;

    let tag = get_tag(args, &installed)?;

    if tag.is_empty() {
        return Ok(());
    }

    if tag.as_ref() == "--all" {
        println!(
            "Removing all CMake version and contents in '{}'.",
            cvm_home.to_str().unwrap_or("")
        );

        std::fs::remove_dir_all(cvm_home).map_err(|error| {
            Rc::from(format!(
                "Failed to remove $HOME/.cvm directory. ({})",
                error
            ))
        })?;
    }

    println!("Checking if the version we are removing is in use...");
    let version = current_version(cvm_home)?;
    if *version == *tag {
        println!(
            "Cannot remove this version since its the current version selected.\nSwitch before removing. Or use --all to remove everything"
        );
        return Ok(());
    }

    println!("Checking if the version we are trying to remove is installed...");
    let mut found_index = None;
    for i in 0..installed.len() {
        if installed[i] == tag {
            println!("Version found...");
            found_index = Some(i);
            break;
        }
    }

    if found_index.is_none() {
        println!(
            "Selected CMake version v{} is not installed. Therefore cannot be removed",
            tag
        );
        return Ok(());
    }

    let index = found_index.unwrap();
    let version = installed.remove(index);

    println!("Removing version installation...");
    let dir_to_rm = cvm_home
        .join(crate::CVM_BINS)
        .join(format!("cmake-{}", version));

    std::fs::remove_dir_all(dir_to_rm.clone()).map_err(|error| {
        Rc::from(format!(
            "Failed to remove '{}'. ({})",
            dir_to_rm.to_str().unwrap_or(""),
            error
        ))
    })?;

    let file_path = cvm_home.join(crate::CVM_INSTALLED);
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .open(file_path)
        .map_err(|error| Rc::from(format!("Failed to open cvm_current file. ({})", error)))?;

    file.set_len(0)
        .map_err(|error| Rc::from(format!("Failed to clear cvm_installed file. ({})", error)))?;

    for version in installed {
        let version = format!("{}\n", version.as_ref());

        file.write(version.as_bytes()).map_err(|error| {
            Rc::from(format!(
                "Failed to write current install to file. ({})",
                error
            ))
        })?;
    }

    println!("Successfully removed CMake v{}.", tag);

    Ok(())
}

fn get_tag(args: &Vec<Rc<str>>, installed: &Vec<Rc<str>>) -> Result<Rc<str>, Rc<str>> {
    if args.len() == 3 {
        return Ok(args[2].clone().into());
    }

    if installed.is_empty() {
        println!("There are no CMake version that is installed that we track.");
        return Ok("".into());
    }

    let message = String::from("Please select a cmake verson to remove:");
    let mut builder = List::<String>::new(message);

    for i in 0..installed.len() {
        builder = builder.add_item(installed[i].as_ref(), installed[i].to_string());
    }

    let result = builder.inquire();

    match result {
        Ok(selected) => Ok(selected.into()),
        Err(inq_msg) => match inq_msg {
            InquiryMessage::CloseRequested => {
                println!("\nSession was canceled. Exiting...");
                Ok("".into())
            }
            _ => Err("Inquiry failed. exiting session".into()),
        },
    }
}

use std::io::Write;
use std::path::Path;
use std::rc::Rc;

use term_inquiry::{InquiryMessage, List};

use crate::releases::current_version;
use crate::releases::installed;
