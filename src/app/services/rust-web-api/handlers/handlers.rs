use tokio::sync::oneshot::Receiver;
use ultimate_rust_service::foundation::server::server;

pub fn load_web_handlers(
    addr: String,
    port: u16,
    shutdown_signal: Receiver<()>,
) -> Result<server::Axum, axum::Error> {
    // Here we add our business level middleware

    // Here we add our routes based on version (prefixed)

    // Here we lastly create our new server, and return to main for it to block the application
    // As stated before, this will be in a seperate thread so we can have multiple senders potentially
    // gracefully shut down the application.

    let server = server::new(server::Config {
        web_address: addr,
        port: port,
        router: axum::Router::new(),
        tracer: String::from(""),
        shutdown_signal: shutdown_signal,
    });

    Ok(server)
}
