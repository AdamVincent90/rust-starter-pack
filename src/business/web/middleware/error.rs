use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse};

use crate::{
    business::system::validation::validation::RequestError, dependency::logger::logger::Logger,
};

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

    // TODO - need to find a way to pull in the request error type that has been propergated here in order
    // TODO - to handle it and return a response.

    if response.status().as_u16() != 200 {
        context.log.warn_w(
            "status code not 200 returned, so we should properly handle API errors",
            Some(()),
        );
    }

    Ok(response)
}
