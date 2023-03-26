use sqlx::postgres;
use ultimate_rust_service::business;
use ultimate_rust_service::foundation::logger::logger;
use ultimate_rust_service::foundation::server::server::{self, Axum};

use super::versions::version_one::users;

// This is where we provide all our packages, and options to prepare our web handler with the relevant
// features they require to perform business operations.
pub struct HandlerConfig<'a> {
    pub web_address: String,
    pub web_port: u16,
    pub debug_address: String,
    pub debug_port: u16,
    // Custom Packages
    pub logger: &'a logger::Logger,
    pub db: postgres::PgPool,
}

// fn prepare_web_handler() loads and initiates our web server with our app level configurations and logic, ready
// to perform business level tasks.
pub fn prepare_web_handler(config: &HandlerConfig) -> Result<Axum, axum::Error> {
    // Here we add our business level middleware

    // Here we add our routes based on version (prefixed)

    // Here we lastly create our new server, and return to main for it to block the application
    // As stated before, this will be in a seperate thread so we can have multiple senders potentially
    // gracefully shut down the application.

    let preload = server::new(server::Config {
        web_address: config.web_address.clone(),
        port: config.web_port,
        router: axum::Router::new(),
        tracer: String::from(""),
    });

    let axum = load_version_one_routes(&config, preload).unwrap_or_else(|err| {
        return Err(err).unwrap();
    });

    Ok(axum)
}

fn load_version_one_routes(config: &HandlerConfig, axum: Axum) -> Result<Axum, axum::Error> {
    let user_handlers = users::UserHandlers {
        version: "v1".to_string(),
        user_core: business::core::user::user::new_core(&config.logger, &config.db),
    };

    //user_handlers.register_user_routes(axum.router, "/v1");

    Ok(axum)
}
