pub mod foundation;

use std::{io::Error, thread};

use signal_hook::{consts::SIGTERM, iterator::Signals};

/// main.rs acts as the entrypoint for our start up and shutdown for this executable.
#[actix_web::main]
async fn main() {
    // Begin application.

    // Logger configuration to allow this application to define our custom logger.
    let logger_config = foundation::logger::logger::Config {
        name: String::from("RUST-LOGGER"),
        max_log_level: log::LevelFilter::Debug,
    };

    // Logger configuration to allow this application to create our custom logger.
    let log = foundation::logger::logger::new_logger(logger_config);

    log.info_w("STARTING RUST SERVICE", Some(()));

    // We now begin the start up function in order to bundle our modules, and setup
    // our services ready to listen to events. We bubble any errors up during our start up
    // sequence in order for them to be handled for our shutdown function.
    start_up(&log).await.unwrap_or_else(|err| {
        // We use unwrap or else as we want do not return a result from this function,
        // thereforece we can use a callack to log the error, and start the shutdown process.
        log.error_w(
            "error during start up sequence, shutting down application gracefully. Error : ",
            Some(err.to_string()),
        );

        // Shut down process to attempt graceful shutdown of our application.
        shut_down(&log, err).unwrap_or_else(|err| {
            log.error_w(
                "error during shutdown process, exiting application. Error : ",
                Some(err.to_string()),
            )
        })
    })
}

// fn start_up() performs all related start up configuration to load our service,
// this is where you will initialise your modules to then be used within your application.
async fn start_up(
    logger: &foundation::logger::logger::Logger,
) -> Result<(), Box<dyn std::error::Error>> {
    // ---------------------------------------
    // start up configuration.

    // ---------------------------------------
    // custom postgres configuration.
    let database_config = foundation::database::database::Config {
        db_host: String::from("postgres"),
        db_port: 5432,
        db_username: String::from("postgres"),
        db_password: String::from("example"),
        db_schema: String::from("postgres"),
        max_connections: 10,
        enable_ssl: sqlx::postgres::PgSslMode::Disable,
    };

    // ---------------------------------------
    // custom postgres initialisation. (error propergated back up, otherwise continue)
    let db = match foundation::database::database::open_postgres_database(database_config).await {
        Ok(db) => db,
        Err(err) => {
            return Err(err)?;
        }
    };

    logger.info_w("database loaded", Some(()));

    // ---------------------------------------
    // custom actix web server configuration.
    let web_config = foundation::server::server::Config {
        web_address: String::from("localhost"),
        port: 80,
    };

    // ---------------------------------------
    // custom actix web server initialisation. (error propergated back up, otherwise continue)
    let _server = match foundation::server::server::new_actix_server(web_config).await {
        Ok(server) => server,
        Err(err) => {
            return Err(err)?;
        }
    };

    logger.info_w("server loaded", Some(()));

    // Create a signal that listens to SIGTERM events.
    let mut signals = Signals::new(&[SIGTERM])?;

    // We spawn a thread and passes in any mutable values defined.
    // This needs to be improved.
    thread::spawn(move || {
        Ok(for sig in signals.forever() {
            println!("sigterm event {} triggered, now exiting program.", sig);
            return Err(());
        })
    });

    // This loop is here to currently block the application from finishing, and currently just pings to
    // the postgres database, and axtix web server.
    // The aim here is for the actix web server, debug web server (in seperate threads) to block until
    // A signal is sent back that warrants a graceful termination of the program.
    loop {
        foundation::database::database::ping_postgres_server(&db, &logger, 5)
            .await
            .unwrap_or_else(|err| logger.error_w("status check failed", Some(err)));

        foundation::server::server::ping_actix_server(&logger, 5)
            .await
            .unwrap_or_else(|err| logger.error_w("server ping failed", Some(err)));
    }
}

// fn shut_down() acts as the shutdown sequence to safely and gracefully shutdown our application.
fn shut_down(
    logger: &foundation::logger::logger::Logger,
    err: Box<dyn std::error::Error>,
) -> Result<(), Error> {
    // We currently just log here to notify we are about to start the shut down process.
    logger.info_w(
        "attempting graceful shutdown of servic : reason ",
        Some(err),
    );

    // We currently just return OK. But this is where we will make sure to stop threads, workers
    // and clean up the application.

    Ok(())
}
