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
    let database_config = foundation::database::database::Config {
        db_host: String::from("postgres"),
        db_port: 5432,
        db_username: String::from("postgres"),
        db_password: String::from("example"),
        db_schema: String::from("postgres"),
        max_connections: 10,
        enable_ssl: sqlx::postgres::PgSslMode::Disable,
    };

    let db = foundation::database::database::open_postgres_database(database_config)
        .await
        .unwrap_or_else(|err| {
            log.error_w(
                "could not pool database connection : error",
                Some(err.to_string()),
            );
            std::process::exit(0)
        });

    log.info_w("database loaded", Some(()));

    // ---------------------------------------
    // custom web server configuration support.

    let web_config = foundation::server::server::Config {
        web_address: String::from("localhost"),
        port: 80,
    };

    let server = foundation::server::server::new_actix_server(web_config)
        .await
        .unwrap_or_else(|err| {
            log.error_w("establishing web server", Some(err.to_string()));
            std::process::exit(0)
        });

    log.info_w("server loaded", Some(()));

    server.resume().await;

    // Add some shutdown logic using channels

    // This is just a test to simulate a ping to the database.
    loop {
        thread::sleep(Duration::from_secs(5));
        foundation::database::database::ping_connection(&db, &log, 5)
            .await
            .unwrap_or_else(|err| log.error_w("status check failed", Some(err)));

        foundation::server::server::ping_axtix_server(&log, 5)
            .await
            .unwrap_or_else(|err| log.error_w("server ping failed", Some(err)));
    }
}
