use env_logger;
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
        let output = self.to_json(message, origin, "INFO").to_string();
        log::info!("\n\x1b[32m{}\x1b[32m", output);
    }
    // Custom WARNING log that formats to JSON.
    pub fn warn_w(&self, message: &str, origin: Option<&str>) {
        let output = self.to_json(message, origin, "WARN").to_string();
        log::warn!("\n\x1b[33m{}\x1b[33m", output);
    }
    // Custom ERROR log that formats to JSON.
    pub fn error_w(&self, error_message: &str, origin: Option<&str>) {
        let output = self.to_json(error_message, origin, "ERROR").to_string();
        log::error!("\n\x1b[91;1m{}\x1b[91;1m", output)
    }
    // Custom DEBUG log that formats to JSON.
    pub fn debug_w(&self, message: &str, origin: Option<&str>) {
        let output = self.to_json(message, origin, "DEBUG").to_string();
        log::debug!("\n\x1b[34m{}\x1b[34m", output)
    }
    // fn to_json() passes in the logging arguments, and formats into more readable, and a log that can be serialised.
    fn to_json(
        &self,
        log_message: &str,
        origin: Option<&str>,
        level: &str,
    ) -> serde_json::value::Value {
        let origin = match origin {
            Some(origin) => origin,
            None => "No Origin Specified.",
        };

        let message_key = format!("{}_message", level.to_lowercase());

        serde_json::json!(
            {"origin": Some(origin), message_key: log_message, "logger_name": self.name, "level": level.to_uppercase()})
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
