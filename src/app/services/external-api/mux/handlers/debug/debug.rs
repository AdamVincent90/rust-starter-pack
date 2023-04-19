use axum::{extract::State, response::IntoResponse, Json};
use rust_starter_pack::{
    domain::system::error::error::SystemError,
    lib::{database::database, web::server::liveness_check},
};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct DebugContext {
    pub version: String,
    pub db: PgPool,
    pub web_address: String,
    pub web_port: u16,
}

pub async fn check_database_status(
    State(context): State<Arc<DebugContext>>,
) -> Result<impl IntoResponse, SystemError> {
    if let Err(err) = database::readiness_check(&context.db, 10).await {
        return Err(SystemError::new(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            err.to_string(),
        ));
    };

    Ok(Json("database OK"))
}

pub async fn check_web_server_status(
    State(context): State<Arc<DebugContext>>,
) -> Result<impl IntoResponse, SystemError> {
    if let Err(err) = liveness_check(context.web_address.clone(), context.web_port, 10).await {
        return Err(SystemError::new(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            err.to_string(),
        ));
    };

    Ok(Json("web server status OK"))
}
