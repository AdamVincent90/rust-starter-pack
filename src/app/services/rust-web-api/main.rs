mod config;
mod handlers;

use signal_hook::consts::SIGTERM;
use tokio::sync::oneshot;
use ultimate_rust_service::foundation::database::database;
use ultimate_rust_service::foundation::logger::logger;

use std::io::Error;

use signal_hook::{consts::SIGINT, iterator::Signals};

use crate::config::Conf;

// The main config struct, this contains your derived structs that can be mapped from a .env.
// Defaults of these can also be provided when initialising the struct in fn start_up()
// All of your custom configurations should be applied in config.rs, derive Serializable, and
// Impl Conf in order to allow .env mappings and defaults.
pub struct AppConfig {
    pub app: config::AppSettings,
    pub web: config::WebSettings,
    pub db: config::DatabaseSettings,
}

/// main.rs acts as the entrypoint for our start up and shutdown for this executable.
#[tokio::main]
async fn main() {
    // Begin application.

    // Logger configuration to allow this application to define our custom logger.
    let logger_config = logger::Config {
        name: String::from("RUST-WEB-API"),
        max_log_level: log::LevelFilter::Debug,
    };

    // Logger configuration to allow this application to create our custom logger.
    let log = logger::new_logger(logger_config);

    log.info_w("STARTING RUST SERVICE", Some(()));

    // We now begin the start up function in order to bundle our modules, and setup
    // our services ready to listen to events. We bubble any errors up during our start up
    // sequence in order for them to be handled for our shutdown function.
    start_up(&log).await.unwrap_or_else(|err| {
        // Shut down process to attempt graceful shutdown of our application.
        log.error_w(
            "error during start up sequence, shutting down application gracefully. Error : ",
            Some(err.to_string()),
        );

        // Shut down process to attempt graceful shutdown of our application.
        // We use unwrap or else as we want do not return a result from this function,
        // therefore we can use a callack to log the error, and start the shutdown process.
        shut_down(&log, err).unwrap_or_else(|err| {
            log.error_w(
                "error during shutdown process, exiting application. Error : ",
                Some(err.to_string()),
            );
            std::process::exit(1);
        })
    });
}

// fn start_up() performs all related start up configuration to load our service,
// this is where you will initialise your modules to then be used within your application.
async fn start_up(logger: &logger::Logger) -> Result<(), Box<dyn std::error::Error>> {
    // ---------------------------------------
    // start up configuration.

    // Define default application configuration.
    let default_config = AppConfig {
        // Default configuration can be added like so.
        app: config::AppSettings {
            version: String::from("0.0.1"),
            environment: String::from("development"),
        }
        .load_from_env(&logger, "")?, // And then we can override from env if needed.
        web: config::WebSettings {
            address: String::from("0.0.0.0"),
            port: 80,
            debug_address: String::from("0.0.0.0"),
            debug_port: 4080,
        }
        .load_from_env(&logger, "WEB")?,
        db: config::DatabaseSettings {
            host: String::from("postgres"),
            port: 5432,
            username: String::from("postgres"),
            password: String::from("example"),
            schema: String::from("postgres"),
        }
        .load_from_env(&logger, "DB")?,
    };

    // -----------------------------------------------------------
    // Custom postgres configuration, and initialsation.
    let database_config = database::Config {
        db_host: default_config.db.host,
        db_port: default_config.db.port,
        db_username: default_config.db.username,
        db_password: default_config.db.password,
        db_schema: default_config.db.schema,
        max_connections: 2,
        enable_ssl: sqlx::postgres::PgSslMode::Disable,
    };

    let db = match database::open_postgres_database(database_config).await {
        Ok(db) => db,
        Err(err) => {
            return Err(err)?;
        }
    };

    logger.info_w("postgres database loaded", Some(()));

    // Now all custom modules have been loaded, we can now start creating threads for our web server, signals, and any other
    // threads we would like to add.

    // Firstly we will create a one time signal and thread that sends a signal to the receiver upon a SIGINT OR SIGTERM event.
    let (signal_send, signal_receive) = oneshot::channel();

    // This is where we pass in our signals into a new thread. This thread simply loops over the signal forever, until one
    // of SIGTERM or SIGINT signal has been matched. Because we are using a loop here, we need to let the borrow checker
    // know that signal_send is a Option, and if we can take from it, we send a signal back to the receiver.
    tokio::spawn(async move {
        let mut signal_send = Some(signal_send);
        let mut signal_interupt = Signals::new(&[SIGINT, SIGTERM]).unwrap();
        for signal in signal_interupt.forever() {
            match signal {
                SIGINT => {
                    if let Some(signal_send) = signal_send.take() {
                        signal_send.send(()).ok();
                    }
                }
                SIGTERM => {
                    if let Some(signal_send) = signal_send.take() {
                        signal_send.send(()).ok();
                    }
                }
                _ => {
                    continue;
                }
            }
        }
    });

    // Finally, we can set up our web and debug server, we also create a onetime channel for graceful shutdowns.
    let (web_send, web_recv) = oneshot::channel();
    let (debug_send, debug_recv) = oneshot::channel();

    let handler_config = handlers::handlers::HandlerConfig {
        web_address: default_config.web.address,
        web_port: default_config.web.port,
        debug_address: default_config.web.debug_address,
        debug_port: default_config.web.debug_port,
        logger: logger,
        db: db,
    };

    // Finally, we create our new rust app, that passes in all the relevant configurations from start up.
    // Ownership is transferred to new_rust_app.
    let (web_server, debug_server) = handlers::handlers::new_handlers(handler_config)
        .unwrap_or_else(|err| {
            logger.error_w("could not prepare web handlers", Some(&err));
            return Err(err).unwrap();
        });

    // This will also contain a seperate debug server, serving on a different port and ofcourse thread.
    web_server.run_sever(web_send).unwrap_or_else(|err| {
        return Err(err).unwrap();
    });

    // Once we run the server, this will now be ran in a seperate thread, as above, the channel we send will notifiy the below
    // select statement.
    debug_server.run_sever(debug_send).unwrap_or_else(|err| {
        return Err(err).unwrap();
    });

    logger.info_w("axum server loaded", Some(()));

    // This is where we will block the main thread until one of these signals is received back. Once a signal has been sent
    // From either, our packages, or from sigint, we then attempt to gracefully shutdown the application, if an error occurs
    // from then, we will attempt to shutdown the program ungracefully, and then a solution to stop these should be implemented.
    tokio::select! {
            val = web_recv => {
                logger.info_w("signal received from web server, starting graceful shutdown", Some(()));
                match val {
                    Ok(_) => {
                        return Ok(());
                    },
                    Err(err) => {
                        return Err(Box::new(err));
                    }
                };
            },
            val = debug_recv => {
                logger.info_w("signal received from debug server, starting graceful shutdown", Some(()));
                match val {
                    Ok(_) => {
                        return Ok(());
                    },
                    Err(err) => {
                        return Err(Box::new(err));
                    }
                };
            },
            val = signal_receive => {
              logger.info_w("signal received from sigint, starting graceful shutdown", Some(()));
                match val {
                    Ok(_) => {
                        return Ok(());
                    },
                    Err(err) => {
                        return Err(Box::new(err));
                    }
                };
            },
    };
}

// fn shut_down() acts as the shutdown sequence to safely and gracefully shutdown our application.
fn shut_down(logger: &logger::Logger, err: Box<dyn std::error::Error>) -> Result<(), Error> {
    // We currently just log here to notify we are about to start the shut down process.
    logger.info_w(
        "attempting graceful shutdown of servic : reason ",
        Some(err),
    );

    // We currently just return OK. But this is where we will make sure to stop threads, workers
    // and clean up the application.

    Ok(())
}
