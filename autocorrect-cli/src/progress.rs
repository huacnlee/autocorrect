use colored::*;
use std::io::{self, Write};

pub fn ok(show: bool) {
    if show {
        print!("{}", ".".green());
        io::stdout().flush().unwrap();
    }
}

pub fn err(show: bool) {
    if show {
        print!("{}", ".".red());
        io::stdout().flush().unwrap();
    }
}
