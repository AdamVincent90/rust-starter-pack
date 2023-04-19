use crate::{domain::system::error::error::SystemError, lib::database};
use axum::http::header;
use axum::response::IntoResponse;
use axum::{extract::State, http::Request, middleware::Next};
use sqlx::PgPool;

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
) -> Result<impl IntoResponse, SystemError> {
    // Pre Handler Logic

    // Extract request headers
    let headers = request.headers().clone();

    // Extract request params to store into the audit logs table.

    // * Host
    let host = headers.get(header::HOST).and_then(|val| val.to_str().ok());
    // * User Agent
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|val| val.to_str().ok());
    // * IP Address
    // ! ( Only used for logging for potential threats, never use for ill purposes!!!!! )
    let ip_address = headers
        .get("X-Forwarded-For")
        .and_then(|val| val.to_str().ok());
    // * Uuid (This should be done during tracing and used across everywhere. Not generated here).
    let request_uuid = uuid::Uuid::new_v4();
    // * Path
    let path = request.uri().to_string();

    // We await the response for other data.

    let response = next.run(request).await;

    // Post Handler Logic

    // * Status code
    let status_code = response.status();

    let query = "
    INSERT INTO audit_logs (user_agent, web_path, host_address, origin_ip_address, request_uuid, status_code)
    VALUES($1, $2, $3, $4, $5, $6);
    ";

    let statement = sqlx::query(query)
        .bind(user_agent)
        .bind(path)
        .bind(host)
        .bind(ip_address)
        .bind(request_uuid.as_hyphenated().to_string())
        .bind(format!("{}", status_code.as_u16()));

    // Insert a new user record into the database using the mutate_statement()
    if let Err(err) = database::database::mutate_statement(&context.db, statement).await {
        return Err(SystemError::new(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            err.to_string(),
        ));
    }

    Ok(response)
}
