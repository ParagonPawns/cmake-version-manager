pub fn install_or_switch(version: &str, cvm_home: &str) {
    let current = match currently_installed(cvm_home) {
        Ok(version) => version,
        Err(error) => {
            log_error(
            &format!("Failed to get currently installed version. {}", error)
            );
            return
        }
    };
    
    if current == version {
        println!(
            "Version already installed and set. If not working make sure PATH \
            is set correctly"
        );
        return
    }

    let args = vec![String::from("cvm"), String::from("install"), version.to_string()];
    install::install_version(&args, cvm_home);
    set_current_install(cvm_home, version);
}

use crate::install;
use crate::log::log_error;
use crate::releases:: {
    currently_installed,
    set_current_install
};