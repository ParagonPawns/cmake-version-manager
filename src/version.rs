pub fn display_version(args: &Vec<String>) {
    if args.len() > 2 {
        AnsiBuilder::new()
            .color().fg().yellow()
            .text("warning: ")
            .reset_attributes()
            .text("Extra arguments will not be read with --version/-v.")
            .println();
    }

    AnsiBuilder::new()
        .color().fg().green()
        .text("cvm: ")
        .color().fg().blue()
        .text("v")
        .text(env!("CARGO_PKG_VERSION"))
        .println()
        .color().fg().gray()
        .text("(cmake version manager)")
        .reset_attributes()
        .println();
}

use ansi_builder::AnsiBuilder;
