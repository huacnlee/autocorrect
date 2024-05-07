use owo_colors::OwoColorize;
use std::{
    io::{self, Write},
    time::SystemTime,
};

use crate::{cli::Cli, logger::SystemTimeDuration as _};

pub fn ok(cli: &Cli) {
    if cli.quiet || !cli.formatter.is_diff() {
        return;
    }

    write!(io::stdout(), "{}", ".".green()).unwrap();
}

pub fn warn(cli: &Cli) {
    if cli.quiet || !cli.formatter.is_diff() {
        return;
    }

    write!(io::stdout(), "{}", ".".yellow()).unwrap();
}

pub fn err(cli: &Cli) {
    if cli.quiet || !cli.formatter.is_diff() {
        return;
    }

    write!(io::stdout(), "{}", ".".red()).unwrap();
}

/// print time spend from start_t to now
pub fn finish(_cli: &Cli, start_t: SystemTime) {
    log::info!(
        "AutoCorrect spend time: {}\n",
        format!("{}ms", start_t.elapsed_millis()).bright_black()
    );
}
