use std::{
    convert::Infallible,
    future::Future,
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use awc::{error::SendRequestError, http::StatusCode, ResponseBody};
use axum::{
    body::{Body, HttpBody},
    handler::Handler,
    http::{status::InvalidStatusCode, Request},
    middleware::{from_fn, Next},
    response::Response,
    routing::{on, Route},
    Router,
};
use axum::{
    response::IntoResponse,
    routing::{get, MethodFilter},
};
use signal_hook::low_level::exit;
use tokio::sync::oneshot::Sender;
use tower::{Layer, ServiceBuilder};

use super::middleware::{self, Middleware};

#[derive(Clone)]
// The main Axum struct.
pub struct Axum {
    pub web_address: String,
    pub port: u16,
    pub router: Router,
    pub tracer: String,
}

// Configuration struct for our Axum.
pub struct Config {
    pub web_address: String,
    pub port: u16,
    pub router: Router,
    pub tracer: String,
}

// fn new() returns a new Axum struct.
pub fn new(config: Config) -> Axum {
    Axum {
        web_address: config.web_address,
        port: config.port,
        router: config.router,
        tracer: config.tracer,
    }
}

// Axum contains functionalities to run the server.
impl Axum {
    // aync fn run_server() starts the axum server, ready to listen to requests, and then handle based on the axum
    // configuration provided.
    pub fn run_sever(self, shutdown_signal: Sender<()>) -> Result<(), Box<dyn std::error::Error>> {
        // We want to initialise a tracer (This could be run in a seperate thread on a seperate server)

        // Add App level middleware

        // We provide a base route to ping.
        // All other routes should be added to the server prior to running
        // Routes added prior will contain route level middleware.
        let routes = self.router.route(
            "/",
            get(|| async {
                format!("ping successful");
            }),
        );

        // Attempt to parse string of loopback address to u8.
        let host = IpAddr::from_str(&self.web_address).unwrap_or_else(|err| {
            println!("{}", err);
            exit(1);
        });

        // Create a new socket.
        let socket_address = SocketAddr::new(host, self.port);

        tokio::spawn(async move {
            // Bind our socket with the provided socket address.
            // We also then start serving the web server, this will then block the application from running.
            // We also add a signal receiver that listens to a sender signal. Once that signal is received,
            // We can then unblock the application to shutdown gracefully.
            let serving = axum::Server::bind(&socket_address).serve(routes.into_make_service());

            // Here we just wait for the blocked application to either receive a signal, or an error that requires the server to exit.
            // This allows us to atleast propergate the error the call stack.
            if let Err(_) = serving.await {
                shutdown_signal.send(()).ok();
            };
        });

        Ok(())
    }
}

// async fn ping_server() does a ping to the server to validate is liveness.
pub async fn ping_server(max_attempts: u8) -> Result<(), SendRequestError> {
    // We use awc as the client to send requests for now.
    let client = awc::Client::default();

    // Based on the number of attempts provided, we keep pinging the server until this limit is reached
    // Once it has, we return an error.
    for i in 1..=max_attempts {
        match client.get("http://127.0.0.1:80").send().await {
            Ok(_) => {
                break;
            }
            Err(err) => {
                if i == max_attempts {
                    return Err(err);
                }
            }
        };
    }

    Ok(())
}
