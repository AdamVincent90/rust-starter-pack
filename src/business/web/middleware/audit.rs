use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

// AuditContext contains all the state required to succefully audit a request.
#[derive(Clone)]
pub struct AuditContext {
    pub auth: String,
}

// This one will use an extractor and post handler logic to add to audit logs.
pub async fn audit<B>(
    State(context): State<AuditContext>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // Pre Handler Logic

    let response = next.run(request).await;

    // Post Handler Logic

    println!("{}", context.auth);

    Ok(response)
}
