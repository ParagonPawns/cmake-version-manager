mod help;
mod install;
mod install_or_switch;
mod list;
mod log;
mod releases;
mod remove;
mod setup;
mod switch;
mod utils;
mod version;

const CVM_BINS: &'static str = "bins";
const CVM_DIR: &'static str = ".cvm";
const CVM_CACHE: &'static str = "cvm_cache";
const CVM_INSTALLED: &'static str = "cvm_installed";
const CVM_CURRENT: &'static str = "cvm_current";

#[cfg(unix)]
const HOME_ENV_STR: &'static str = "HOME";
#[cfg(windows)]
const HOME_ENV_STR: &'static str = "USERPROFILE";

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

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
        _ => {
            if utils::is_version_number(&args[1]) {
                if args.len() != 2 {
                    log_warning(
                        "There are other arguments detected after cmake version.\nIgnoring arguments after version.",
                    );
                }

                install_or_switch::install_or_switch(&args[1], &cvm_home)?;
                return Ok(());
            }

            log_error(
                "The first argument does not match whith any option we support.\nPlease use 'cvm --help' to view all possible options.\nGiven options: ",
            );

            for i in 1..args.len() {
                print!("{} ", args[i]);
            }
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<Rc<str>> = std::env::args().map(|arg| arg.into()).collect();

    if args.len() == 1 {
        log::log_error("Did not get any options for command. Execute 'cvm --help' for options");
        return;
    }

    let cvm_home = match std::env::var(HOME_ENV_STR) {
        Ok(path) => Path::new(&path).join(CVM_DIR),
        Err(error) => {
            log::log_error(&format!("Failed to find $HOME path. ({})", error));
            return;
        }
    };

    if let Err(error) = setup::setup_cvm(&cvm_home) {
        log::log_error(error.as_ref());
        log::log_error("Failed to set up cvm for use.");
        return;
    }

    if let Err(error) = process_arguments(&args, &cvm_home) {
        log::log_error(error.as_ref());
    }
}

use std::path::Path;
use std::rc::Rc;

use curl::easy::{Handler, WriteError};
use log::{log_error, log_warning};
