use std::future::Future;

use axum::{
    http::{status::InvalidStatusCode, Request},
    middleware::Next,
    response::Response,
};

pub struct Middleware {
    pub next: dyn Fn(Request<()>, Next<()>) -> Result<Response, InvalidStatusCode>,
}
