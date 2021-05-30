mod install;
mod list;
mod log;
mod releases;
mod setup;
mod switch;
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

    let cvm_home = match std::env::var("HOME") {
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

    if args[1] == "list" {
        list::list_releases(&args, &cvm_home);
        return
    }

    if args[1] == "install" {
        install::install_version(&args, &cvm_home);
        return
    }

    if args[1] == "switch" {
        switch::switch_version(&args, &cvm_home);
        return
    }
}

use curl::easy::{ Handler, WriteError };
use switch::switch_version;
