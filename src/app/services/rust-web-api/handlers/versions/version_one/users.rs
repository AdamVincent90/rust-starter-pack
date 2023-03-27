use std::sync::Arc;

// The version 1 routes for handling users.
use axum::{extract::State, response::IntoResponse, Json};
use axum_macros::debug_handler;
use ultimate_rust_service::business::{
    self, core::user::models::V1PostUser, system::validation::validation::RequestError,
};
use validator::Validate;
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
pub async fn v1_get_user_by_id(State(context): State<Arc<UserContext>>) -> impl IntoResponse {
    println!("in handler");
    let result = context
        .user_core
        .v1_get_users_by_id()
        .unwrap_or_else(|err| Err(err).unwrap());

    Json(result)
}

#[debug_handler]
pub async fn v1_post_user(
    State(context): State<Arc<UserContext>>,
    Json(user): Json<V1PostUser>,
) -> Result<impl IntoResponse, RequestError> {
    println!("in handler");

    match user.validate() {
        Err(err) => {
            return Err(RequestError::new(
                axum::http::StatusCode::BAD_REQUEST,
                err.to_string(),
            ))
        }
        _ => (),
    };

    let result = context
        .user_core
        .v1_get_users_by_id()
        .unwrap_or_else(|err| Err(err).unwrap());

    Ok(Json(result))
}
