pub fn log_error(msg: &str) {
    AnsiBuilder::new()
        .color().fg().bright_red()
        .text("error: ")
        .reset_attributes()
        .text(msg)
        .println();
}

use ansi_builder::AnsiBuilder;
