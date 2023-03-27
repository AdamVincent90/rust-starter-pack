use axum::{
    http::status::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct RequestError {
    pub status_code: StatusCode,
    pub message: String,
}

impl RequestError {
    pub fn new(status_code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status_code,
            message: message.into(),
        }
    }
}

impl IntoResponse for RequestError {
    fn into_response(self) -> Response {
        // its often easiest to implement `IntoResponse` by calling other implementations
        (self.status_code, self.message).into_response()
    }
}
