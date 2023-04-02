use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;
use ultimate_rust_service::business::{
    self, core::user::V1PostUser, system::validation::validation::RequestError,
};
use validator::Validate;

// UserContext contains any state required when it comes to working with user operations.
#[derive(Clone)]
pub struct UserContext {
    pub version: String,
    pub user_core: business::core::user::user::UserCore,
}

// * Any errors returned from handler functions, will be caught and then processed in middleware.

// fn v1_get_users() is the main handler for (GET /v1/users)
pub async fn v1_get_users(State(context): State<Arc<UserContext>>) -> impl IntoResponse {
    // Once validated, or doing any logic involving the request, we send to our core entrypoint function.
    let result = context
        .user_core
        .v1_get_users()
        .await
        .unwrap_or_else(|err| Err(err).unwrap());

    Json(result)
}

// fn v1_get_user_by_id() is the main handler for (GET /v1/users/{id})
pub async fn v1_get_user_by_id(State(context): State<Arc<UserContext>>) -> impl IntoResponse {
    // Once validated, or doing any logic involving the request, we send to our core entrypoint function.
    let result = context
        .user_core
        .v1_get_users_by_id()
        .await
        .unwrap_or_else(|err| Err(err).unwrap());

    Json(result)
}

// fn v1_get_user_by_id() is the main handler for (POST /v1/users)
pub async fn v1_post_user(
    State(context): State<Arc<UserContext>>,
    Json(user): Json<V1PostUser>,
) -> Result<impl IntoResponse, RequestError> {
    // Before sending off to core logic, for request bodies, we should validate it.
    match user.validate() {
        Err(err) => {
            return Err(RequestError::new(
                axum::http::StatusCode::BAD_REQUEST,
                err.to_string(),
            ))
        }
        _ => (),
    };

    // Once validated, or doing any logic involving the request, we send to our core entrypoint function.
    context
        .user_core
        .v1_post_user(user)
        .await
        .unwrap_or_else(|err| Err(err).unwrap());

    // Here, we simply send back status code 201.
    Ok(axum::http::StatusCode::CREATED)
}
