use super::versions::version_one::users;
use axum::routing::get;
use axum::Json;
use sqlx::postgres;
use std::sync::Arc;
use ultimate_rust_service::business;
use ultimate_rust_service::foundation::logger::logger;
use ultimate_rust_service::foundation::server::server::{self, Axum};

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
pub fn new_rust_app(config: HandlerConfig) -> Result<(Axum, Axum), axum::Error> {
    // TODO - Application level stuff
    // App level middlewares
    // Debug Axum Server
    // Web Axum Server

    // Any premade checks and logic before loading our servers.

    // Here we add our routes based on version (prefixed)
    let v1_axum = initialise_v1_web_routing(&config);
    let debug_axum = initialise_debug_routing(&config);
    Ok((v1_axum, debug_axum))
}

fn initialise_debug_routing(config: &HandlerConfig) -> Axum {
    let debug_router = axum::Router::new();
    let debug_router = debug_router // We provide a base route to ping.
        .route(
            "/",
            get(|| async {
                let message = "ping successful";
                println!("{}", message);
                Json(message)
            }),
        );
    server::new(server::Config {
        web_address: config.debug_address.clone(),
        port: config.debug_port,
        router: debug_router,
        tracer: String::from(""),
    })
}

fn initialise_v1_web_routing(config: &HandlerConfig) -> Axum {
    let version = "v1";

    // Create user handler that will acts as the context for users routes.
    let user_context = users::UserContext {
        version: version.to_string(),
        user_core: business::core::user::user::new_core(&config.logger, &config.db),
    };

    // Build our router for users.
    let user_router = axum::Router::new()
        // GET ( /v1/users )
        .route(
            format!("/{}{}", version, "/users").as_str(),
            get(users::v1_get_users),
        )
        // GET ( /v1/users/:id )
        .route(
            format!("/{}{}", "/v1", "/users/:id").as_str(),
            get(users::v1_get_users_by_id),
        )
        // Create context for users using Arc.
        .with_state(Arc::new(user_context));

    // Here we lastly create our new server, and return to main for it to block the application
    // As stated before, this will be in a seperate thread so we can have multiple senders potentially
    // gracefully shut down the application.
    server::new(server::Config {
        web_address: config.web_address.clone(),
        port: config.web_port,
        router: axum::Router::new().merge(user_router), // Here we merge our routers that contain different context state, and middlewares.
        tracer: String::from(""),
    })
}
