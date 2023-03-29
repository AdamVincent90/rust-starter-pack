// Lumber will be an eventual tool that will create store and core functions with a simple command.
use std::{env, io::Error, process::exit};

mod commands;

use log::LevelFilter;
use ultimate_rust_service::foundation::logger::logger::{self, Config, Logger};

struct ConfigArgs<'a> {
    command: &'a str,
    name: &'a str,
}

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

    let config_args = ConfigArgs {
        command: &args[0],
        name: &args[1],
    };

    if let Err(err) = run(&logger, config_args) {
        logger.error_w("error during lumber process : error ", Some(err));
        exit(1)
    }
}

fn run(logger: &Logger, args: ConfigArgs) -> Result<(), Error> {
    logger.info_w("intiating selected command : ", Some(args.command));

    match args.command {
        "core" => commands::core::create_core(logger, args.command, args.name)
            .unwrap_or_else(|err| return Err(err).unwrap()),
        "store" => commands::store::create_store(logger, args.command, args.name)
            .unwrap_or_else(|err| return Err(err).unwrap()),
        "worker" => commands::worker::create_worker(logger, args.command, args.name)
            .unwrap_or_else(|err| return Err(err).unwrap()),
        _ => {}
    }

    Ok(())
}
