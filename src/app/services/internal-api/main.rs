mod config;
mod rpc;

use crate::config::Conf;
use crate::rpc::rpc::new_rpc;
use rust_starter_pack::domain::system::auth::auth::AuthConfig;
use rust_starter_pack::lib::logger::logger;
use rust_starter_pack::{domain::system::auth::auth, lib::database::database};
use serde::Serialize;
use signal_hook::consts::SIGTERM;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::io::Error;
use tokio::sync::oneshot;

// The main config struct, this contains your derived structs that can be mapped from a .env.
// Defaults of these can also be provided when initialising the struct in fn start_up()
// All of your custom configurations should be applied in config.rs, derive Serializable, and
// Impl Conf in order to allow .env mappings and defaults.
#[derive(Serialize)]
pub struct AppConfig {
    pub app: config::AppSettings,
    pub web: config::GrpcSettings,
    pub db: config::DatabaseSettings,
    pub auth: config::AuthSettings,
}

/// main.rs acts as the entrypoint for our start up and shutdown for this executable.
#[tokio::main]
async fn main() {
    // Begin application.

    // Logger configuration to allow this application to define our custom logger.
    let logger_config = logger::Config {
        name: String::from("INTERNAL-API"),
        max_log_level: log::LevelFilter::Debug,
    };

    // Logger configuration to allow this application to create our custom logger.
    let log = logger::new_logger(logger_config);

    log.info_w("STARTING INTERNAL SERVICE", Some("INTERNAL API MAIN"));

    // We now begin the start up function in order to bundle our modules, and setup
    // our services ready to listen to events. We bubble any errors up during our start up
    // sequence in order for them to be handled for our shutdown function.
    if let Err(err) = start_up(&log).await {
        // Shut down process to attempt graceful shutdown of our application.
        log.error_w(
            format!(
                "error during start up sequence, shutting down application gracefully. Error : {}",
                err.to_string()
            )
            .as_str(),
            Some("INTERNAL API MAIN"),
        );

        // Shut down process to attempt graceful shutdown of our application.
        // therefore we can use a callack to log the error, and start the shutdown process.
        if let Err(err) = shut_down(&log, err) {
            log.error_w(
                format!(
                    "error during shutdown process, exiting application. Error : {}",
                    err.to_string()
                )
                .as_str(),
                Some("INTERNAL API MAIN"),
            );
            std::process::exit(1);
        }
    };
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
        web: config::GrpcSettings {
            address: String::from("0.0.0.0"),
            port: 50051,
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
        auth: config::AuthSettings {
            enabled: false,
            key_id: String::from("some-uuid"),
            public_key: String::from("******"),
        }
        .load_from_env(&logger, "AUTH")?,
    };

    // -----------------------------------------------------------
    // Log default configuration
    match serde_json::to_string(&default_config) {
        Ok(json) => {
            logger.info_w(json.as_str(), Some("Rust API startup"));
        }
        Err(err) => logger.warn_w(
            format!(
                "could not serialise default config, skipping.. : {}",
                err.to_string()
            )
            .as_str(),
            Some("Rust API startup"),
        ),
    }

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

    logger.info_w("postgres database loaded", Some("Rust Web API Start Up"));

    // -----------------------------------------------------------
    // Auth support
    let auth_config = AuthConfig {
        enabled: default_config.auth.enabled,
        key_id: default_config.auth.key_id,
        signing_method: jsonwebtoken::Algorithm::RS256,
        db: db.clone(),
    };

    let auth = auth::new(auth_config);

    logger.info_w("auth config loaded", Some("Rust Web API Start Up"));

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
    let (grpc_send, grpc_recv) = oneshot::channel();

    // Finally, we create our new internal app, that passes in all the relevant configurations from start up.
    // Ownership is transferred to new_rust_app.

    let rpc_config = rpc::rpc::RpcConfig {
        environment: default_config.app.environment,
        web_address: default_config.web.address,
        port: default_config.web.port,
        auth: auth,
        db: db,
        log: &logger,
    };

    let tonic = new_rpc(rpc_config);

    // Once we run the server, this will now be ran in a seperate thread, as above, the channel we send will notifiy the below
    // select statement.
    tonic.run_server(grpc_send)?;

    logger.info_w("grpc tonic server loaded", Some("Internal API Start Up"));

    // This is where we will block the main thread until one of these signals is received back. Once a signal has been sent
    // From either, our packages, or from sigint, we then attempt to gracefully shutdown the application, if an error occurs
    // from then, we will attempt to shutdown the program ungracefully, and then a solution to stop these should be implemented.
    tokio::select! {
          val = grpc_recv => {
                logger.info_w("signal received from grpc server, starting graceful shutdown", Some("Rust Web API Start Up"));
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
              logger.info_w("signal received from sigint, starting graceful shutdown", Some("Rust Web API Start Up"));
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
        format!(
            "attempting graceful shutdown of service : reason {}",
            err.to_string()
        )
        .as_str(),
        Some("Internal API Shut Down"),
    );

    // We currently just return OK. But this is where we will make sure to stop threads, workers
    // and clean up the application.

    Ok(())
}
