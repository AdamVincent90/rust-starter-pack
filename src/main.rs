pub mod foundation;

use std::env;

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
        db_host: String::from("localhost:5434"),
        db_username: String::from("postgres"),
        db_password: String::from("example"),
        db_schema: String::from("postgres"),
        max_connections: 10,
    };

    let db = foundation::database::database::new_postgres_database(database_confing)
        .await
        .unwrap_or_else(|err| {
            log.error_w(
                "could not pool database connection : error",
                Some(err.to_string()),
            );
            std::process::exit(0)
        });

    // ---------------------------------------
    // Query database to check if available (Will be moved)
    let row: (bool,) = sqlx::query_as("SELECT true")
        .fetch_one(&db)
        .await
        .unwrap_or_else(|err| {
            log.error_w(
                "cannot check database connection has loaded",
                Some(err.to_string()),
            );
            std::process::exit(0)
        });

    println!("{}", row.0);
}

// Will contain all logic to start up the service.
fn start_up() {}

// Will contain all the logic to end the service gracefully.
fn shut_down() {}
