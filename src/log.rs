pub fn log_error(msg: &str) {
    AnsiBuilder::new()
        .color().fg().bright_red()
        .text("error: ")
        .reset_attributes()
        .text(msg)
        .println();
}

pub fn log_warning(msg: &str) {
    AnsiBuilder::new()
        .color().fg().bright_yellow()
        .text("warning: ")
        .reset_attributes()
        .text(msg)
        .println();
}

use ansi_builder::AnsiBuilder;
