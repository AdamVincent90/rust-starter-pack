pub mod foundation;

use std::{env, thread, time::Duration};

/// main.rs acts as the startup and shutdown sequence for the main service.
#[actix_web::main]
async fn main() {
    // Set lrust log env var to max of debug.
    env::set_var("RUST_LOG", "debug");

    // ---------------------------------------
    // custom Logger configuration support.
    let logger_config = foundation::logger::logger::Config {
        name: String::from("RUST-LOGGER"),
        max_log_level: log::LevelFilter::Debug,
    };

    let log = foundation::logger::logger::new_logger(logger_config);

    log.info_w("STARTING RUST SERVICE", Some(()));

    // ---------------------------------------
    // custom postgres configuration support.
    let database_confing = foundation::database::database::Config {
        db_host: String::from("localhost"),
        db_port: 5434,
        db_username: String::from("postgres"),
        db_password: String::from("example"),
        db_schema: String::from("postgres"),
        max_connections: 10,
        enable_ssl: sqlx::postgres::PgSslMode::Disable,
    };

    let db = foundation::database::database::open_postgres_database(database_confing)
        .await
        .unwrap_or_else(|err| {
            log.error_w(
                "could not pool database connection : error",
                Some(err.to_string()),
            );
            std::process::exit(0)
        });

    // This is just a test to simulate a ping to the database.
    loop {
        thread::sleep(Duration::from_secs(5));
        foundation::database::database::ping_connection(&db, &log, 5)
            .await
            .unwrap_or_else(|err| log.error_w("status check failed", Some(err)));
    }
}

// Will contain all logic to start up the service.
fn start_up() {}

// Will contain all the logic to end the service gracefully.
fn shut_down() {}
