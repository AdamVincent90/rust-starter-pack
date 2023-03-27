use std::sync::Arc;

// The version 1 routes for handling users.
use axum::{extract::State, response::IntoResponse, Json};
use axum_macros::debug_handler;
use ultimate_rust_service::business;
#[derive(Clone)]
// Contains the the core business logic for users, and potentially other things.
pub struct UserContext {
    pub version: String,
    pub user_core: business::core::user::user::Core,
}

// Below needs to provide Extractors to pull in our User Handler contexts.

#[debug_handler]
pub async fn v1_get_users(State(context): State<Arc<UserContext>>) -> impl IntoResponse {
    println!("in handler");
    let result = context
        .user_core
        .v1_get_users()
        .unwrap_or_else(|err| Err(err).unwrap());

    Json(result)
}

#[debug_handler]
pub async fn v1_get_users_by_id(State(context): State<Arc<UserContext>>) -> impl IntoResponse {
    println!("in handler");
    let result = context
        .user_core
        .v1_get_users_by_id()
        .unwrap_or_else(|err| Err(err).unwrap());

    Json(result)
}
