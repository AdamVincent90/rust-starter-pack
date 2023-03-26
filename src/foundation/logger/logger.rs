use env_logger;
use log;
use std::fmt::Debug;

// To clean up and improve.

#[derive(Clone)]
pub struct Logger {
    name: String,
}

pub struct Config {
    pub name: String,
    pub max_log_level: log::LevelFilter,
}

impl Debug for Logger {
    fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result {
        f.debug_struct("Logger").field("name", &self.name).finish()
    }
}

impl Logger {
    pub fn info_w<T: Debug>(&self, message: &str, args: Option<T>) {
        log::info!(
            "\n\x1b[32mlogger name: {} || logger info message : {} || args : {:?}\x1b[32m\n",
            self.name,
            message,
            args
        )
    }

    pub fn warn_w<T: Debug>(&self, message: &str, args: Option<T>) {
        log::warn!(
            "\n\x1b[33mlogger name: {} || logger warn message : {} || args : {:?}\x1b[33m\n",
            self.name,
            message,
            args
        )
    }

    pub fn error_w<T: Debug>(&self, message: &str, args: Option<T>) {
        log::error!(
            "\n\x1b[91;1mlogger name: {} || logger error message : {} || args : {:?}\x1b[91;1m\n",
            self.name,
            message,
            args
        )
    }

    pub fn debug_w<T: Debug>(&self, message: &str, args: Option<T>) {
        log::debug!(
            "\n\x1b[34mlogger name: {} || logger debug message : {} || args : {:?}\x1b[34m\n",
            self.name,
            message,
            args
        )
    }
}

// fn new_logger() creates a new logger that sets logging to standard output.
pub fn new_logger(config: Config) -> Logger {
    // Allows logging to support standard outputs.
    env_logger::init();

    // Sets the desired log levels we would like to log out to the standard output.
    log::set_max_level(config.max_log_level);

    // Create a Logger struct that contains functions for this specific logger.
    Logger { name: config.name }
}
