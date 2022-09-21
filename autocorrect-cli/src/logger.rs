use log::{Metadata, Record};
use std::{
    io::{self, Write},
    time::SystemTime,
};

#[derive(Debug)]
pub struct Logger;

#[allow(dead_code)]
static LOGGER: Logger = Logger;

impl Logger {
    /// Create a new logger that logs to stderr and initialize it as the
    /// global logger. If there was a problem setting the logger, then an
    /// error is returned.
    pub fn init(level: log::LevelFilter) -> Result<(), log::SetLoggerError> {
        log::set_logger(&LOGGER).map(|()| log::set_max_level(level))
    }
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let metadata = record.metadata();

        if !self.enabled(metadata) {
            return;
        }

        println!("{}", record.args());
    }

    fn flush(&self) {
        io::stdout().flush().unwrap();
    }
}

pub trait SystemTimeDuration {
    /// Time elapsed duration in ms
    fn elapsed_millis(&self) -> u128;
}

impl SystemTimeDuration for SystemTime {
    fn elapsed_millis(&self) -> u128 {
        self.elapsed().unwrap_or_default().as_millis()
    }
}
