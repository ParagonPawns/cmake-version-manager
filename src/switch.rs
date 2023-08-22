pub fn switch_version(args: &Vec<Rc<str>>, cvm_home: &Path) -> Result<(), Rc<str>> {
    let installed = installed(cvm_home)?;

    let tag = get_tag(args, &installed)?;

    if tag.is_empty() {
        return Ok(());
    }

    // We don't mind if no versions are currently installed.
    let current = current_version(cvm_home).unwrap_or("".into());

    if *tag == *current {
        println!("CMake version {} is already selected.", tag);
        return Ok(());
    }

    println!("Checking to see if version is already installed...");
    if !is_installed(&tag, &installed) {
        println!(
            "CMake version {0} is not installed. Please run 'cvm install {0}'",
            tag
        );
        return Ok(());
    }

    if !current.is_empty() {
        let from = cvm_home.join(crate::CVM_BINS).join(crate::CVM_CURRENT_DIR);
        let to = cvm_home
            .join(crate::CVM_BINS)
            .join(format!("cmake-{}", current));

        std::fs::rename(from, to)
            .map_err(|error| Rc::from(format!("Failed to rename directory. ({})", error)))?;
    }

    println!("Switching...");
    switch(&tag, cvm_home)?;

    println!("Successfully switch to CMake v{}", tag);
    Ok(())
}

fn get_tag(args: &Vec<Rc<str>>, installed: &Vec<Rc<str>>) -> Result<Rc<str>, Rc<str>> {
    if args.len() == 3 {
        return Ok(args[2].clone());
    }

    if installed.is_empty() {
        println!("There are no installed versions of CMake that we are tracking.");
        return Ok("".into());
    }

    let mut builder = List::<Rc<str>>::new("Please select a cmake verson to switch to:");

    for i in 0..installed.len() {
        builder = builder.add_item(installed[i].as_ref(), installed[i].clone());
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

pub fn switch(version: &str, cvm_home: &Path) -> Result<(), Rc<str>> {
    let from = cvm_home
        .join(crate::CVM_BINS)
        .join(format!("cmake-{}", version));
    let to = cvm_home.join(crate::CVM_BINS).join(crate::CVM_CURRENT_DIR);

    std::fs::rename(from, to)
        .map_err(|error| Rc::from(format!("Failed to rename directory. ({})", error)))?;

    set_current_install(cvm_home, version)
}

use std::path::Path;
use std::rc::Rc;

use term_inquiry::{InquiryMessage, List};

use crate::releases::{current_version, installed, is_installed, set_current_install};
