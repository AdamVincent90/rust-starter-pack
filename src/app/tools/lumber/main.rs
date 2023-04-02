// Lumber will be an eventual tool that will create store and core functions with a simple command.
use std::{env, io::Error, process::exit};

mod commands;

use log::LevelFilter;
use rust_starter_pack::foundation::logger::logger::{self, Config, Logger};

fn main() {
    let logger = logger::new_logger(Config {
        name: String::from("LUMBER"),
        max_log_level: LevelFilter::Debug,
    });

    logger.info_w("starting lumber tool", Some(()));

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        logger.error_w("you must provide command : error ", Some(()));
        exit(1)
    }

    let command = &args[1];
    let name = &args[2];
    let opts = &args[3..];

    if let Err(err) = run(&logger, command, name, opts) {
        logger.error_w("error during lumber process : error ", Some(err));
        exit(1)
    }
}

fn run(logger: &Logger, command: &str, name: &str, opts: &[String]) -> Result<(), Error> {
    logger.info_w("intiating selected command : ", Some(command));

    match command {
        "core" => commands::core::create_core(logger, command, name, opts)
            .unwrap_or_else(|err| return Err(err).unwrap()),
        "store" => commands::store::create_store(logger, command, name)
            .unwrap_or_else(|err| return Err(err).unwrap()),
        "worker" => commands::worker::create_worker(logger, command, name)
            .unwrap_or_else(|err| return Err(err).unwrap()),
        _ => {}
    }

    Ok(())
}
