pub fn list_releases(args: &Vec<String>, cvm_home: &str) {
    let releases = match cached_releases(cvm_home) {
        Ok(releases) => releases,
        Err(error) => {
            log_error(&format!("Failed to get cached releases. ({})", error));
            return
        }
    };


    let current = match currently_installed(cvm_home) {
        Ok(curr) => curr,
        Err(error) => {
            log_error(&format!("Failed to get current version. ({})", error));
            return
        }
    };

    let installed = match installed(cvm_home) {
        Ok(installed) => installed,
        Err(error) => {
            log_error(&format!("Failed to get installed versions.({})", error));
            return
        }
    };

    if args.len() == 2 {
        println!("Current available releases (last 10):");
        for i in 0..10 {
            print_release(&releases[i], &current, &installed);
        }
        return
    }

    if args.len() >= 3 && args[2] != "--all" && args[2] != "-a" {
        log_error("Option 'list' does not contain expected optional flags: \
        --list, -l");
        return
    }

    println!("{:?}\n{:?}", installed, releases);
    for release in releases {
        print_release(&release, &current, &installed);
    }
}

fn print_release(release: &str, current: &str, installed: &Vec<String>) {
    let is_installed = is_installed(release, installed);
    let text = if release == current && is_installed {
        "(installed | selected)"
    } else if is_installed {
        "(installed)"
    } else {
        ""
    };

    println!("{}-{}-{}", release, current, is_installed);

    AnsiBuilder::new()
        .text(&format!("    {} ", release))
        .color().fg().green()
        .text(text)
        .reset_attributes()
        .println();
}

use ansi_builder::AnsiBuilder;

use crate::log::log_error;
use crate::releases::{
    cached_releases,
    currently_installed,
    installed,
    is_installed
};
