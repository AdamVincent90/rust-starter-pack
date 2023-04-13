use crate::business::system::{auth::auth::Auth, error::error::RequestError};
use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse};

// AuditContext contains all the state required to succefully audit a request.
#[derive(Clone)]
pub struct AuthContext {
    pub auth: Auth,
}

pub async fn authenticate<B>(
    State(mut context): State<AuthContext>,
    request: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, RequestError> {
    // Pre Handler Logic

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

    if let Err(err) = context.auth.authenticate(parts[1].to_string()) {
        return Err(RequestError::new(
            axum::http::StatusCode::FORBIDDEN,
            format!("You are not authenticated : {}", err.as_str()),
        ));
    }

    println!("claims: {:?}", context.auth.claims);

    // Do something with claims, how to safely share between state?

    let response = next.run(request).await;

    // Post Handler Logic

    Ok(response)
}

pub async fn authorise<B>(
    roles: Option<Vec<String>>,
    State(context): State<AuthContext>,
    request: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, RequestError> {
    // Pre Handler Logic

    if let Err(err) = context.auth.authorise(&context.auth.claims, roles) {
        return Err(RequestError::new(
            axum::http::StatusCode::UNAUTHORIZED,
            format!("You are not authorised : {}", err.as_str()),
        ));
    }

    let response = next.run(request).await;

    // Post Handler Logic

    Ok(response)
}
