use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse};

use crate::{business::system::error::error::RequestError, dependency::logger::logger::Logger};

#[derive(Clone)]
pub struct ErrorContext {
    pub log: Logger,
}

pub async fn error<B>(
    State(context): State<ErrorContext>,
    request: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, RequestError> {
    // Pre Handler Logic

    let response = next.run(request).await;

    let status = response.status();

    match status.as_u16() {
        200..=299 => Ok(response),
        _ => {
            // We know its an error now so we can properly decode the body to return the messages we want too.

            let status = response.status();

            let data = match hyper::body::to_bytes(response.into_body()).await {
                Ok(data) => data,
                Err(err) => {
                    context
                        .log
                        .error_w("could not convert to bytes in error middleware", Some(err));
                    return Err(RequestError::new_internal_server_error());
                }
            };

            let data = match std::str::from_utf8(&data) {
                Ok(data) => data,
                Err(err) => {
                    context
                        .log
                        .error_w("could no read bytes in error middleware ", Some(err));
                    return Err(RequestError::new_internal_server_error());
                }
            };

            // We can now log the error message to the console, so we know the reason for the 500 error, but the user does not.
            context.log.warn_w(&data.to_string(), Some(()));

            // We only return the bytes if the status code is 400-499.

            match status.as_u16() {
                400..=499 => {
                    return Err(RequestError::new(status, data.to_string()));
                }
                _ => {
                    return Err(RequestError::new_internal_server_error());
                }
            }
        }
    }
}
