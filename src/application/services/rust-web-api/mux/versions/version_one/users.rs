use axum::{
    extract::{Extension, Path, State},
    response::IntoResponse,
    Json,
};
use rust_starter_pack::business::{
    self, core::user::V1PostUser, system::error::error::RequestError, web::state::state,
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

// * Handlers deal with three things.
// * 1. Validate the request
// * 2. Call the core function (Passing in any extensions or context)
// * 3. Returning the outcome.

// fn v1_get_users() is the main handler for (GET /v1/users)
pub async fn v1_get_users(
    Extension(state): Extension<state::WebState>,
    State(context): State<Arc<UserContext>>,
) -> Result<impl IntoResponse, RequestError> {
    let state = &state.read().await;

    // Once validated, or doing any logic involving the request, we send to our core entrypoint function.
    let result = match context.user_core.get_all(&state.auth).await {
        Ok(result) => result,
        Err(err) => {
            return Err(err);
        }
    };

    Ok(Json(result))
}

// fn v1_get_user_by_id() is the main handler for (GET /v1/users/{id})
pub async fn v1_get_user_by_id(
    Extension(state): Extension<state::WebState>,
    State(context): State<Arc<UserContext>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, RequestError> {
    let state = &state.read().await;
    // Once validated, or doing any logic involving the request, we send to our core entrypoint function.
    let result = match context.user_core.get_by_id(&state.auth, id).await {
        Ok(result) => result,
        Err(err) => {
            return Err(err);
        }
    };

    Ok(Json(result))
}

// fn v1_get_user_by_id() is the main handler for (POST /v1/users)
pub async fn v1_post_user(
    Extension(state): Extension<state::WebState>,
    State(context): State<Arc<UserContext>>,
    Json(user): Json<V1PostUser>,
) -> Result<impl IntoResponse, RequestError> {
    let state = &state.read().await;
    // Before sending off to core logic, for request bodies, we should validate it.
    if let Err(err) = user.validate() {
        return Err(RequestError::new(
            axum::http::StatusCode::BAD_REQUEST,
            err.to_string(),
        ));
    }

    // Once validated, or doing any logic involving the request, we send to our core entrypoint function.
    if let Err(err) = context.user_core.create(&state.auth, user).await {
        return Err(err);
    }

    // Here, we simply send back status code 201.
    Ok(axum::http::StatusCode::CREATED)
}
