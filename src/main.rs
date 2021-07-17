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

const CVM_DIR: &'static str = "/.cvm";
const CVM_CACHE: &'static str = "/cvm_cache";
const CVM_INSTALLED: &'static str = "/cvm_installed";
const CVM_CURRENT: &'static str = "/cvm_current";

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        log::log_error("Did not get any options for command. Execute 'cvm --help' for options");
        return
    }

    if args[1] == "--version" || args[1] == "-v" {
        version::display_version(&args);
        return
    }

    if args[1] == "--help" || args[1] == "-h" {
        help::dislay_help();
        return
    }

    #[cfg(unix)]
    let home = "HOME";
    #[cfg(windows)]
    let home = "USERPROFILE";

    let cvm_home = match std::env::var(home) {
        Ok(path) => path + CVM_DIR,
        Err(error) => {
            log::log_error(&format!("Failed to find $HOME path. ({})", error));
            return
        }
    };

    if !setup::setup_cvm(&cvm_home) {
        log::log_error("Failed to set up cvm for use.");
        return
    }

    match args[1].as_str() {
        "current" => {
            match releases::currently_installed(&cvm_home) {
                Ok(version) =>
                    println!("Currently selected CMake version is v{}", version),
                Err(error) =>
                    log::log_error(
                        &format!("Failed to retrieve currently selected CMake install. ({})", error)
                    )
            }
            return
        },
        "list" => {
            list::list_releases(&args, &cvm_home);
            return
        },
        "install" => {
            install::install_version(&args, &cvm_home);
            return
        },
        "remove" => {
            remove::remove(&args, &cvm_home);
            return
        },
        "switch" => {
            switch::switch_version(&args, &cvm_home);
            return
        },
        _ => { 
            if utils::is_version_number(&args[1]) {
                if args.len() != 2 {
                    log_warning(
                        "There are other arguments detected after cmake \
                        version. Ignoring arguments after version."
                    );
                }

                install_or_switch::install_or_switch(&args[1], &cvm_home);
                return
            }

            log_error(
                "The first argument does not match whith any option we support. \
                please use 'cvm --help' to view all possible options. Given \
                options: "
            );

            for i in 1..args.len() {
                print!("{} ", args[i]);
            }
        }
    }
}

use curl::easy::{ Handler, WriteError };
use log::{ log_error, log_warning };