use super::versions::version_one::users;
use axum::routing::{get, post};
use axum::Json;
use rust_starter_pack::business;
use rust_starter_pack::foundation::logger::logger;
use rust_starter_pack::foundation::server::server::{self, Axum};
use sqlx::postgres;
use std::sync::Arc;

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

// fn new_handlers() creates two isolated web services, a debug service, and web service.
// Web service acts as the main service that handles incoming requests, and processes them.
// Debug service acts as the debug server that will contain metrics, and alerting.
pub fn new_handlers(config: HandlerConfig) -> Result<(Axum, Axum), axum::Error> {
    // TODO - The below things need to be done before initialising routing.
    // TODO - App level middlewares added in order to wrap over all routes.
    // TODO - Anything else that requires before initialising routing.

    // Here we add our routes based on version (prefixed)
    let v1_axum = initialise_v1_web_routing(&config);
    let debug_axum = initialise_debug_routing(&config);
    Ok((v1_axum, debug_axum))
}

// fn initialise_debug_routing creates our debug routes, for now, this just contains a root path that pings itself.
// This initial route will help in understanding if the debug service is experiencing any down time.
// But this service can also provide liveness, and readiness checks for our main web server.
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

// fn initialise_v1_web_routing creates our main web service that contains routes that handle our core business logic.
// Each routing group has its own context that contains any configs and core packages required to perform operations.
// This flow helps to segregate our code and to make sure that ownership is brought down the stack in a consistent
// manner.
fn initialise_v1_web_routing(config: &HandlerConfig) -> Axum {
    // The version of our routes.
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
            format!("/{}{}", version, "/users/:id").as_str(),
            get(users::v1_get_user_by_id),
        )
        .route(
            format!("/{}{}", version, "/users").as_str(),
            post(users::v1_post_user),
        )
        // Create context for users using Arc.
        .with_state(Arc::new(user_context));

    // Here we lastly create our new server, and return to main for it to block the application
    // As stated before, this will be in a seperate thread so we can have multiple senders potentially
    // gracefully shut down the application.
    server::new(server::Config {
        web_address: config.web_address.clone(),
        port: config.web_port,
        // Here we merge our routers that contain different context state, and middlewares.
        router: axum::Router::new().merge(user_router),
        tracer: String::from(""),
    })
}
