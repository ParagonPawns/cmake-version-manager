pub fn install_or_switch(version: &str, cvm_home: &Path) -> Result<(), Rc<str>> {
    // We don't mind if no versions are currently installed.
    let current = current_version(cvm_home).unwrap_or("".into());

    if current.as_ref() == version {
        println!(
            "Version already installed and set. If not working make sure PATH is set correctly"
        );
        return Ok(());
    }

    let args = vec!["cvm".into(), "install".into(), version.into()];

    install::install_version(&args, cvm_home)?;
    set_current_install(cvm_home, version)?;

    Ok(())
}

use std::path::Path;
use std::rc::Rc;

use crate::install;
use crate::releases::{current_version, set_current_install};
