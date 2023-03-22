mod config;

use ultimate_rust_service::foundation::database::database;
use ultimate_rust_service::foundation::logger::logger;
use ultimate_rust_service::foundation::server::server;

use std::{io::Error, thread};

use config as app_config;
use signal_hook::{consts::SIGINT, iterator::Signals};

/// main.rs acts as the entrypoint for our start up and shutdown for this executable.
#[actix_web::main]
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
    let default_config = app_config::AppConfig {
        app: app_config::AppSettings {
            version: "0.0.1".to_string(),
            environment: "development".to_string(),
        },
        web: app_config::WebSettings {
            address: "localhost".to_string(),
            port: 80,
        },
        db: app_config::DatabaseSettings {
            host: "postgres".to_string(),
            port: 5432,
            username: "postgres".to_string(),
            password: "example".to_string(),
            schema: "postgres".to_string(),
        },
    };

    // Here, we send out default config to our load from env function to map settings from env.
    // Defaults should remain, if there is an error, we return back the default config for now.
    // But we should really end the program gracefully.
    let app_config = default_config.load_from_env(&logger)?;

    // ---------------------------------------
    // custom postgres configuration.
    let database_config = database::Config {
        db_host: app_config.db.host,
        db_port: app_config.db.port,
        db_username: app_config.db.username,
        db_password: app_config.db.password,
        db_schema: app_config.db.schema,
        max_connections: 2,
        enable_ssl: sqlx::postgres::PgSslMode::Disable,
    };

    // ---------------------------------------
    // custom postgres initialisation. (error propergated back up, otherwise continue)
    let db = match database::open_postgres_database(database_config).await {
        Ok(db) => db,
        Err(err) => {
            return Err(err)?;
        }
    };

    logger.info_w("postgres database loaded", Some(()));

    println!("{}:{}", app_config.web.address, app_config.web.port);

    // ---------------------------------------
    // custom actix web server configuration.
    let web_config = server::Config {
        web_address: app_config.web.address,
        port: app_config.web.port,
    };

    // ---------------------------------------
    // custom actix web server initialisation. (error propergated back up, otherwise continue)
    let _server = match server::new_actix_server(web_config).await {
        Ok(server) => server,
        Err(err) => {
            return Err(err)?;
        }
    };

    logger.info_w("actix server loaded", Some(()));

    // Create a signal that listens to SIGINT events.
    let mut signals = Signals::new(&[SIGINT])?;
    let signal_handle = signals.handle();

    // We spawn a thread and passes in any mutable values defined.
    // This needs to be improved. Will need to get this working within docker.
    thread::spawn(move || {
        Ok(for sig in signals.forever() {
            match sig {
                SIGINT => {
                    println!("signal event {} triggered, now exiting program.", sig);
                    signals.handle().close();
                    return Err(SIGINT);
                }
                _ => continue,
            }
        })
    });

    // This loop is here to currently block the application from finishing, and currently just pings to
    // the postgres database, and axtix web server.
    // The aim here is for the actix web server, debug web server, and signals (in seperate threads) to block until
    // A signal is sent back that warrants a graceful termination of the program.
    Ok(while !signal_handle.is_closed() {
        database::ping_postgres_server(&db, &logger, 5)
            .await
            .unwrap_or_else(|err| logger.error_w("status check failed", Some(err)));

        server::ping_actix_server(&logger, 5)
            .await
            .unwrap_or_else(|err| logger.error_w("server ping failed", Some(err)));
    })
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
