// The version 1 routes for handling users.

use axum::response::IntoResponse;
use axum_macros::debug_handler;
use ultimate_rust_service::business;
#[derive(Clone)]

// Contains the the core business logic for users, and potentially other things.
pub struct UserHandlers {
    pub version: String,
    pub user_core: business::core::user::user::Core,
}

// Below needs to provide Extractors to pull in our User Handler contexts.

#[debug_handler]
pub async fn v1_get_users() -> impl IntoResponse {}

#[debug_handler]
pub async fn v1_get_users_by_id() -> impl IntoResponse {}
