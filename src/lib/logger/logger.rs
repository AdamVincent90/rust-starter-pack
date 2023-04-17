use env_logger::{self, fmt};
use log;

// A simple custom JSON logger, that is used across the project.
#[derive(Clone)]
pub struct Logger {
    name: String,
}

// Configuration to set the name and max logging level for a given logger.
pub struct Config {
    pub name: String,
    pub max_log_level: log::LevelFilter,
}

impl Logger {
    // Custom INFO log that formats to JSON.
    pub fn info_w(&self, message: &str, origin: Option<&str>) {
        let level = "INFO";
        let output = to_json(message, origin, level).to_string();
        log::info!(
            "\x1b[32m[{} {} (Log Below)]\x1b[32m\n\x1b[32m{}\x1b[32m",
            self.name,
            level,
            output
        );
    }
    // Custom WARNING log that formats to JSON.
    pub fn warn_w(&self, message: &str, origin: Option<&str>) {
        let level = "WARNING";
        let output = to_json(message, origin, level).to_string();
        log::warn!(
            "\x1b[33m[{} {} (Log Below)]\x1b[33m\n\x1b[33m{}\x1b[33m",
            self.name,
            level,
            output
        );
    }
    // Custom ERROR log that formats to JSON.
    pub fn error_w(&self, error_message: &str, origin: Option<&str>) {
        let level = "ERROR";
        let output = to_json(error_message, origin, level).to_string();
        log::error!(
            "\x1b[91;1m[{} {} (Log Below)]\x1b[91;1m\n\x1b[91;1m{}\x1b[91;1m",
            self.name,
            level,
            output
        )
    }
    // Custom DEBUG log that formats to JSON.
    pub fn debug_w(&self, message: &str, origin: Option<&str>) {
        let level = "DEBUG";
        let output = to_json(message, origin, level).to_string();
        log::debug!(
            "\x1b[34m[{} {} (Log Below)]\x1b[34m\n\x1b[34m{}\x1b[34m",
            self.name,
            level,
            output
        )
    }
}

// fn new_logger() creates a new logger that sets logging to standard output.
pub fn new_logger(config: Config) -> Logger {
    // Sets the desired log levels we would like to log out to the standard output.
    log::set_max_level(config.max_log_level);

    // Allows logging to support standard outputs.
    env_logger::Builder::from_default_env()
        .format_indent(None)
        .format_target(false)
        .format_timestamp(Some(fmt::TimestampPrecision::Seconds))
        .init();

    // Create a Logger struct that contains functions for this specific logger.
    Logger { name: config.name }
}

// fn to_json() passes in the logging arguments, and formats into more readable, and a log that can be serialised.
fn to_json(log_message: &str, origin: Option<&str>, level: &str) -> serde_json::value::Value {
    let origin = match origin {
        Some(origin) => origin,
        None => "No Origin Specified.",
    };

    let message_key = format!("{}_message", level.to_lowercase());

    serde_json::json!({
        message_key: log_message,
        "origin": Some(origin)
    })
}
