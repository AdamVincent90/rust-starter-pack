use std::fmt::Error;

use rust_starter_pack::foundation::logger::logger::Logger;

pub fn create_worker(log: &Logger, command: &str, name: &str) -> Result<(), Error> {
    let message = format!("processing {} with name {}", command, name);
    log.info_w(&message, Some(()));
    Ok(())
}
