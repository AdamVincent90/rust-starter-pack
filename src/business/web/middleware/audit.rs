use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

// This one will use an extractor and post handler logic to add to audit logs.

pub async fn audit<B>(request: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    // Pre Handler Logic

    println!("begin request");

    let response = next.run(request).await;

    // Post Handler Logic

    println!("end request");

    Ok(response)
}
