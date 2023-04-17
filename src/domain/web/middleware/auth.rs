use crate::domain::{
    system::{auth::auth::Auth, error::error::RequestError},
    web::state::state::SharedState,
};
use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse, Extension};

// AuthContext contains all the state required to succefully auth a request.
#[derive(Clone)]
pub struct AuthContext {
    pub auth: Auth,
}

pub async fn authenticate<B>(
    Extension(state): Extension<SharedState>,
    State(context): State<AuthContext>,
    request: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, RequestError> {
    // Pre Handler Logic

    // Lock our shared state with write access to update claims to our state on succesful
    // authentication.
    let mut state = state.write().await;

    if context.auth.enabled {
        let token = match request.headers().get(axum::http::header::AUTHORIZATION) {
            Some(token) => token,
            None => {
                return Err(RequestError::new(
                    axum::http::StatusCode::FORBIDDEN,
                    "no authorisation header provided",
                ));
            }
        };

        let token = match token.to_str().ok() {
            Some(token) => token,
            None => {
                return Err(RequestError::new(
                    axum::http::StatusCode::FORBIDDEN,
                    "no authorisation header provided",
                ));
            }
        };

        let parts = token.split_whitespace().collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err(RequestError::new(
                axum::http::StatusCode::FORBIDDEN,
                "no valid authorisation header provided",
            ));
        }

        if let Err(err) = context.auth.authenticate(parts[1].to_string(), &mut state) {
            return Err(RequestError::new(
                axum::http::StatusCode::FORBIDDEN,
                format!("You are not authenticated : {}", err.as_str()),
            ));
        }
    }

    // Because we are calling the next handler, and RWLOCK requires read access for other functions
    // down the stack, we need to drop the lock manually as the scope is not technically ended
    // and the write lock has completed.
    drop(state);

    // Do something with claims, how to safely share between state?

    let response = next.run(request).await;

    // Post Handler Logic

    Ok(response)
}

pub async fn authorise<B>(
    roles: Option<Vec<String>>,
    Extension(state): Extension<SharedState>,
    State(context): State<AuthContext>,
    request: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, RequestError> {
    // Pre Handler Logic

    let state = state.read().await;

    if context.auth.enabled {
        if let Err(err) = context.auth.authorise(&state, roles) {
            return Err(RequestError::new(
                axum::http::StatusCode::UNAUTHORIZED,
                format!("You are not authorised : {}", err.as_str()),
            ));
        }
    }

    // Because we are calling the next handler, and RWLOCK requires read access for other functions
    // down the stack, we need to drop the lock manually as the scope is not technically ended.
    drop(state);

    let response = next.run(request).await;

    // Post Handler Logic

    Ok(response)
}
