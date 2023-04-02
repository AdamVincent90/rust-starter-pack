// Lumber will be an eventual tool that will create store and core functions with a simple command.
use std::{env, io::Error, process::exit};

mod commands;

use log::LevelFilter;
use rust_starter_pack::foundation::logger::logger::{self, Config, Logger};

fn main() {
    env::set_var("RUST_LOG", "debug");

    let logger = logger::new_logger(Config {
        name: String::from("LUMBER"),
        max_log_level: LevelFilter::Debug,
    });

    logger.info_w("starting lumber tool", Some(()));

    let args: Vec<String> = env::args().collect();

    if args.len() <= 2 {
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
        "client" => commands::client::create_client(logger, command, name)
            .unwrap_or_else(|err| return Err(err).unwrap()),
        _ => {
            logger.error_w("unknown command provided. Please see below.", Some(()));
            println!("\n");
            println!("core: Create a core entity: Example: `make lumber core article` ");
            println!(
                "core options: store <options> (db): Example `make lumber core article store db` "
            );
            println!("core options: client <options> (grpc,http): Example `make lumber core article client http` ");
            println!("\n");
            println!("store: Add store functionality to an existing core entity: Example `make lumber store article db`");
            println!("store options: db");
            println!("\n");
            println!("client: Add client functionality to an existing core entity: Example `make lumber client article grpc `");
            println!("client options: http grpc");
        }
    }

    Ok(())
}
