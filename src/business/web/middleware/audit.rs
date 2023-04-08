use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse};
use sqlx::PgPool;

use crate::{business::system::validation::validation::RequestError, dependency::database};

// AuditContext contains all the state required to succefully audit a request.
#[derive(Clone)]
pub struct AuditContext {
    pub db: PgPool,
}

// This one will use an extractor and post handler logic to add to audit logs.
pub async fn audit<B>(
    State(context): State<AuditContext>,
    request: Request<B>,
    next: Next<B>,
) -> impl IntoResponse {
    // Pre Handler Logic

    // Extract request headers
    let headers = request.headers().clone();

    let response = next.run(request).await;

    // Post Handler Logic

    // Extract request params to store into the audit logs table.

    // Host
    let host = headers
        .get(axum::http::header::HOST)
        .and_then(|val| val.to_str().ok());
    // Path
    let path = headers
        .get(axum::http::header::REFERER)
        .and_then(|val| val.to_str().ok());
    // User Agent
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|val| val.to_str().ok());
    // IP Address
    let ip_address = headers
        .get(axum::http::header::FORWARDED)
        .and_then(|val| val.to_str().ok());
    // Uuid
    let request_uuid = uuid::Uuid::new_v4();
    // Status code
    let status_code = response.status();

    let query = "
    INSERT INTO audit_log (user_agent, web_path, host_address, origin_ip_address, request_uuid, status_code)
    VALUES($1, $2, $3, $4, $5, $6);
    ";

    let statement = sqlx::query(query)
        .bind(user_agent)
        .bind(path)
        .bind(host)
        .bind(ip_address)
        .bind(request_uuid.to_string())
        .bind(status_code.as_str());

    // Insert a new user record into the database using the mutate_statement()
    if let Err(err) = database::database::mutate_statement(&context.db, statement).await {
        return Err(RequestError::new(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            err.to_string(),
        ));
    }

    Ok(response)
}
