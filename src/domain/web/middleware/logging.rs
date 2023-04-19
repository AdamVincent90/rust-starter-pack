use crate::{domain::system::error::error::SystemError, lib::logger::logger::Logger};
use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse};

// LoggingContext contains all the state required to succefully log a request.
#[derive(Clone)]
pub struct LoggingContext {
    pub log: Logger,
}

pub async fn logging<B>(
    State(context): State<LoggingContext>,
    request: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, SystemError> {
    // Pre Handler Logic

    let request_message = format!(
        "request starting : METHOD: {} : PATH: {}",
        request.method(),
        request.uri()
    );

    context
        .log
        .info_w(&request_message, Some("Logging Middleware"));

    let response = next.run(request).await;

    // Post Handler Logic

    let response_message = format!("response received: STATUS: {}", response.status().as_str());

    context
        .log
        .info_w(&response_message, Some("Logging Middleware"));

    Ok(response)
}
