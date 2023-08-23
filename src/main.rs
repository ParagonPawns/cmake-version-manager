mod help;
mod install;
mod install_or_switch;
mod list;
mod log;
mod macros;
mod releases;
mod remove;
mod setup;
mod switch;
mod utils;
mod version;

fn process_arguments(args: &Vec<Rc<str>>, cvm_home: &Path) -> Result<(), Rc<str>> {
    match args[1].as_ref() {
        "current" => {
            let version = releases::current_version(&cvm_home)?;
            println!("Currently selected CMake version is v{}", version);
        }
        "list" => {
            list::list_releases(&args, &cvm_home)?;
        }
        "install" => {
            install::install_version(&args, &cvm_home)?;
        }
        "remove" => {
            remove::remove(&args, &cvm_home)?;
        }
        "switch" => {
            switch::switch_version(&args, &cvm_home)?;
        }
        "--help" | "-h" => {
            help::dislay_help();
        }
        "--version" | "-v" => {
            version::display_version(&args);
        }
        x if utils::is_version_number(&x) => {
            if args.len() != 2 {
                log::warning(IGNORING_EXTRA_ARGS_STR);
            }

            install_or_switch::install_or_switch(&args[1], &cvm_home)?;
        }
        _ => {
            let mut msg = UNSUPPORTED_ARG_STR.to_string();
            args.iter()
                .for_each(|arg| msg.push_str(&format!("{} ", arg.as_ref())));

            return Err(msg.into());
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<Rc<str>> = std::env::args().map(|arg| arg.into()).collect();

    if args.len() == 1 {
        log::error(NO_ARGS_DETECTED_STR);
        return;
    }

    let cvm_home = match std::env::var(HOME_ENV_STR) {
        Ok(path) => Path::new(&path).join(CVM_DIR),
        Err(error) => {
            log::error(&format!("Failed to find $HOME path. ({})", error));
            return;
        }
    };

    if let Err(error) = setup::setup_cvm(&cvm_home) {
        log::error(error.as_ref());
        log::error(SETUP_FAILURE_STR);
        return;
    }

    if let Err(error) = process_arguments(&args, &cvm_home) {
        log::error(error.as_ref());
    }
}

const CVM_BINS: &'static str = "bins";
const CVM_DIR: &'static str = ".cvm";
const CVM_CACHE: &'static str = "cvm_cache";
const CVM_INSTALLED: &'static str = "cvm_installed";
const CVM_CURRENT_DIR: &'static str = "current";
const CVM_CURRENT_FILE: &'static str = "cvm_current";
#[cfg(unix)]
const HOME_ENV_STR: &'static str = "HOME";
#[cfg(windows)]
const HOME_ENV_STR: &'static str = "USERPROFILE";
const IGNORING_EXTRA_ARGS_STR: &'static str =
    "There are other arguments detected after cmake version.\nIgnoring arguments after version.";
const NO_ARGS_DETECTED_STR: &'static str =
    "Did not get any options for command. Execute 'cvm --help' for options";
const SETUP_FAILURE_STR: &'static str = "Failed to set up cvm for use.";
const UNSUPPORTED_ARG_STR: &'static str = "The first argument does not match whith any option we support.\nPlease use 'cvm --help' to view all possible options.\nGiven options: ";

use std::path::Path;
use std::rc::Rc;
