use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn logging<B>(request: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    // Pre Handler Logic

    println!("begin request");

    let response = next.run(request).await;

    // Post Handler Logic

    println!("end request");

    Ok(response)
}
