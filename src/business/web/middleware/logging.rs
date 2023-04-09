use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse};

use crate::{business::system::error::error::RequestError, dependency::logger::logger::Logger};

// AuditContext contains all the state required to succefully audit a request.
#[derive(Clone)]
pub struct LoggingContext {
    pub log: Logger,
}

pub async fn logging<B>(
    State(context): State<LoggingContext>,
    request: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, RequestError> {
    // Pre Handler Logic

    let request_message = format!(
        "request starting : METHOD: {} : PATH: {}",
        request.method(),
        request.uri()
    );

    context.log.info_w(&request_message, Some(()));

    let response = next.run(request).await;

    // Post Handler Logic

    let response_message = format!("response received: STATUS: {}", response.status().as_str());

    context.log.info_w(&response_message, Some(()));

    Ok(response)
}
