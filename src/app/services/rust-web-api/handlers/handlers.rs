use axum::routing::MethodFilter;
use axum::{Error, Router};
use sqlx::postgres;
use ultimate_rust_service::foundation::logger::logger;
use ultimate_rust_service::foundation::server::server::{self, Axum};

use super::versions::version_one;

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
pub fn prepare_web_handler(config: &HandlerConfig) -> Result<server::Axum, axum::Error> {
    // Here we add our business level middleware

    // Here we add our routes based on version (prefixed)

    // Here we lastly create our new server, and return to main for it to block the application
    // As stated before, this will be in a seperate thread so we can have multiple senders potentially
    // gracefully shut down the application.

    let server = server::new(server::Config {
        web_address: config.web_address.clone(),
        port: config.web_port,
        router: axum::Router::new(),
        tracer: String::from(""),
    });

    load_version_one_routes(&config, &server);

    Ok(server)
}

fn load_version_one_routes(config: &HandlerConfig, axum: &Axum) {
    let prefix = "/v1";

    let user_group = version_one::users::UserHandlers {
        version: prefix.to_string(),
    };

    // axum.register_route(
    //     MethodFilter::GET,
    //     prefix,
    //     "/users",
    //     version_one::users::UserHandlers::get_users,
    // );
}
