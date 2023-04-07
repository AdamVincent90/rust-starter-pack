use axum::{extract::State, response::IntoResponse, Json};
use rust_starter_pack::business::{
    self, core::user::V1PostUser, system::validation::validation::RequestError,
};
use std::sync::Arc;
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
    let result = match context.user_core.v1_get_users().await {
        Ok(result) => result,
        Err(err) => {
            return Err(err);
        }
    };

    Ok(Json(result))
}

// fn v1_get_user_by_id() is the main handler for (GET /v1/users/{id})
pub async fn v1_get_user_by_id(State(context): State<Arc<UserContext>>) -> impl IntoResponse {
    // Once validated, or doing any logic involving the request, we send to our core entrypoint function.
    let result = match context.user_core.v1_get_users_by_id().await {
        Ok(result) => result,
        Err(err) => {
            return Err(err);
        }
    };

    Ok(Json(result))
}

// fn v1_get_user_by_id() is the main handler for (POST /v1/users)
pub async fn v1_post_user(
    State(context): State<Arc<UserContext>>,
    Json(user): Json<V1PostUser>,
) -> Result<impl IntoResponse, RequestError> {
    // Before sending off to core logic, for request bodies, we should validate it.
    if let Err(err) = user.validate() {
        return Err(RequestError::new(
            axum::http::StatusCode::BAD_REQUEST,
            err.to_string(),
        ));
    }

    // Once validated, or doing any logic involving the request, we send to our core entrypoint function.
    if let Err(err) = context.user_core.v1_post_user(user).await {
        return Err(err);
    }

    // Here, we simply send back status code 201.
    Ok(axum::http::StatusCode::CREATED)
}
