use log::LevelFilter;
use rust_starter_pack::lib::logger::logger;
use rust_starter_pack::lib::logger::logger::Logger;
use std::env;
mod commands;

// Lots of cleaning to do.

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    let logger = logger::new_logger(logger::Config {
        name: String::from("OPENSSL-GEN"),
        max_log_level: LevelFilter::Info,
    });

    logger.info_w("starting open-ssl key generation", Some("SSL main"));

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        logger.error_w("not enough arguments", Some("SSL main"));
        std::process::exit(1)
    }

    let command = &args[1];

    if let Err(err) = run(&logger, command).await {
        logger.error_w(
            format!("error during run process : {}", err.to_string()).as_str(),
            Some("SSL main"),
        );
        std::process::exit(1);
    };
}

async fn run(logger: &Logger, command: &str) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        "keygen" => {
            if let Err(err) = commands::keygen::key_gen(&logger) {
                return Err(err);
            }
        }
        "token" => {
            if let Err(err) = commands::token::make_token(&logger).await {
                return Err(err);
            }
        }
        _ => {
            logger.error_w("unknown command provided.", None);
        }
    }

    Ok(())
}
