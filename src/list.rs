pub fn list_releases(args: &Vec<Rc<str>>, cvm_home: &Path) -> Result<(), Rc<str>> {
    let releases = cached_releases(cvm_home)?;

    // We dont mind if there are no current versions installed.
    let current = current_version(cvm_home).unwrap_or(Rc::from(""));

    // We dont mind if none are installed.
    let installed = installed(cvm_home).unwrap_or(Vec::new());

    if args.len() == 2 {
        println!("Currently available releases (last 10):");
        for i in 0..10 {
            print_release(&releases[i], &current, &installed);
        }

        return Ok(());
    }

    if args.len() >= 3 && args[2].as_ref() != "--all" && args[2].as_ref() != "-a" {
        return Err("Option 'list' must have expected optional flags: --list or -l".into());
    }

    for release in releases {
        print_release(&release, &current, &installed);
    }

    Ok(())
}

fn print_release(release: &str, current: &str, installed: &Vec<Rc<str>>) {
    let is_installed = is_installed(release, installed);
    let text = if release == current && is_installed {
        "(installed | selected)"
    } else if is_installed {
        "(installed)"
    } else {
        ""
    };

    AnsiBuilder::new()
        .text(&format!("    {} ", release))
        .color()
        .fg()
        .green()
        .text(text)
        .reset_attributes()
        .println();
}

use std::path::Path;
use std::rc::Rc;

use ansi_builder::AnsiBuilder;

use crate::releases::{cached_releases, current_version, installed, is_installed};
