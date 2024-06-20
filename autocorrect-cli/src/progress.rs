use owo_colors::OwoColorize;
use std::time::SystemTime;

use crate::{cli::Cli, logger::SystemTimeDuration as _};

pub fn ok(cli: &Cli) {
    if cli.quiet || !cli.formatter.is_diff() {
        return;
    }

    print!("{}", ".".green());
}

pub fn warn(cli: &Cli) {
    if cli.quiet || !cli.formatter.is_diff() {
        return;
    }

    print!("{}", ".".yellow());
}

pub fn err(cli: &Cli) {
    if cli.quiet || !cli.formatter.is_diff() {
        return;
    }

    print!("{}", ".".red());
}

/// print time spend from start_t to now
pub fn finish(cli: &Cli, start_t: SystemTime) {
    if cli.quiet || !cli.formatter.is_diff() {
        return;
    }

    log::info!(
        "AutoCorrect spend time: {}\n",
        format!("{}ms", start_t.elapsed_millis()).bright_black()
    );
}
