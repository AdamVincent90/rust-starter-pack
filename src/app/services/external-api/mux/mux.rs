use super::handlers::debug::debug::{self, DebugContext};
use super::handlers::v1::users::{self, UserContext};
use axum::routing::{get, post};
use axum::{middleware, Extension, Router};
use rust_starter_pack::core::user::user;
use rust_starter_pack::domain::system::auth::auth::{self, StandardClaims};
use rust_starter_pack::domain::web::middleware::audit::{audit, AuditContext};
use rust_starter_pack::domain::web::middleware::auth::{authenticate, AuthContext};
use rust_starter_pack::domain::web::middleware::error::{error, ErrorContext};
use rust_starter_pack::domain::web::middleware::logging::{logging, LoggingContext};
use rust_starter_pack::domain::web::state::state::{MuxState, SharedState};
use rust_starter_pack::lib::logger::logger;
use rust_starter_pack::lib::server::server::{self, Axum};
use sqlx::{postgres, PgPool};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
// Mux acts as the multiplexer in order to configure and create our services that acts as the main layer
// For our business logic.

// This is where we provide all our packages, and options to prepare our web handler with the relevant
// features they require to perform business operations.
pub struct MuxConfig<'a> {
    pub environment: String,
    pub web_address: String,
    pub web_port: u16,
    pub debug_address: String,
    pub debug_port: u16,
    // Custom Packages
    pub logger: &'a logger::Logger,
    pub db: postgres::PgPool,
    pub auth: auth::Auth,
}

// fn new_mux() creates two isolated web services, a debug service, and web service.
// Web service acts as the main service that handles incoming requests, and processes them.
// Debug service acts as the debug server that will contain metrics, and alerting.
pub fn new_mux(config: MuxConfig) -> Result<(Axum, Axum), axum::Error> {
    // Firstly, we create our tracing support.

    // TODO - tracing needs to be more granula and contain actual logging.
    let tracer: Router = Router::new().layer(TraceLayer::new_for_http());

    // Initialise our global web state that is shared across the project.
    // Global state uses A mutex to safely read and write to the state without any side effects.
    let global_state = SharedState::new(RwLock::new(MuxState {
        environment: config.environment.clone(),
        claims: StandardClaims::default(),
    }));

    // * Initialise our v1 routes with our application level middleware, and shared state.
    // Create V1 route handlers.
    let v1_routes = initialise_v1_web_routing(&config)
        .layer(
            // We use ServiceBuilder as this means that the order of middleware is from top to bottom.
            ServiceBuilder::new()
                // * Logging
                .layer(middleware::from_fn_with_state(
                    LoggingContext {
                        log: config.logger.clone(),
                    },
                    logging,
                ))
                // * Error handling
                .layer(middleware::from_fn_with_state(
                    ErrorContext {
                        log: config.logger.clone(),
                    },
                    error,
                ))
                // * Authentication
                .layer(middleware::from_fn_with_state(
                    AuthContext { auth: config.auth },
                    authenticate,
                ))
                // * Auditing
                .layer(middleware::from_fn_with_state(
                    AuditContext {
                        db: config.db.clone(),
                    },
                    audit,
                )),
        )
        .layer(Extension(global_state));

    // Here we lastly create our new muxes, and then return to main in order to block the application
    // As stated before, this will be in a seperate thread so we can have multiple senders potentially
    // that gracefully shut down the application.
    let web_mux = server::new(server::Config {
        web_address: config.web_address.clone(),
        port: config.web_port,
        // Here we merge our versioned routes with our application middleware.
        // It is important to note that route layers (like middleware) need to wrap around routes, so the router
        // needs to contain the routes before the middleware.
        router: tracer.clone().merge(v1_routes),
    });

    // Create Debug route handlers.
    let debug_routes = initialise_debug_routing(config.db.clone()).layer(
        ServiceBuilder::new()
            // * Logging
            .layer(middleware::from_fn_with_state(
                LoggingContext {
                    log: config.logger.clone(),
                },
                logging,
            )),
    );

    let debug_mux = server::new(server::Config {
        web_address: config.debug_address.clone(),
        port: config.debug_port,
        router: tracer.clone().merge(debug_routes),
    });

    Ok((web_mux, debug_mux))
}

// fn initialise_debug_routing creates our debug routes, for now, this just contains a root path that pings itself.
// This initial route will help in understanding if the debug service is experiencing any down time.
// But this service can also provide liveness, and readiness checks for our main web server.
fn initialise_debug_routing(db: PgPool) -> axum::Router {
    let debug_context = DebugContext {
        version: String::from("v1"),
        db: db,
        web_address: String::from("http://host.docker.internal"),
        web_port: 8128,
    };

    let debug_router = axum::Router::new() // We provide a base route to ping.
        .route("/debug/web", get(debug::check_web_server_status))
        .route("/debug/database", get(debug::check_database_status))
        .with_state(Arc::new(debug_context));

    debug_router
}

// fn initialise_v1_web_routing creates our main web service that contains routes that handle our core business logic.
// Each routing group has its own context that contains any configs and core packages required to perform operations.
// This flow helps to segregate our code and to make sure that ownership is brought down the stack in a consistent
// manner.
fn initialise_v1_web_routing(config: &MuxConfig) -> axum::Router {
    // Create user handler that will acts as the context for users routes.
    let user_context = UserContext {
        version: String::from("v1"),
        user_core: user::new_core(&config.logger, &config.db),
    };

    // Build our router for users.
    let user_router = axum::Router::new()
        // * GET ( /v1/users )
        .route("/v1/users", get(users::v1_get_users))
        // * GET ( /v1/users/:id )
        .route("/v1/users/:id", get(users::v1_get_user_by_id))
        // * POST ( /v1/users )
        .route("/v1/users", post(users::v1_post_user))
        // * Create context for users using Arc.
        .with_state(Arc::new(user_context));

    // * More routes go below

    // We return all merged routes here with their own state.
    axum::Router::new().merge(user_router)
}
