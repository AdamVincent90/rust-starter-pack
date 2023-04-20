use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};
use tokio::sync::oneshot::Sender;
use tonic::transport::server::Router;

#[derive(Debug)]
pub struct Tonic {
    pub web_address: String,
    pub port: u16,
    pub router: Router,
}

pub struct TonicConfig {
    pub web_address: String,
    pub port: u16,
    pub router: Router,
}

pub fn new(config: TonicConfig) -> Tonic {
    Tonic {
        web_address: config.web_address,
        port: config.port,
        router: config.router,
    }
}

impl Tonic {
    pub fn run_server(self, shutdown_signal: Sender<()>) -> Result<(), Box<dyn std::error::Error>> {
        // Attempt to parse string of loopback address to u8.
        let host = match IpAddr::from_str(&self.web_address) {
            Ok(host) => host,
            Err(err) => return Err(Box::new(err)),
        };

        // Create a new socket.
        let socket_address = SocketAddr::new(host, self.port);

        tokio::spawn(async move {
            // Bind our socket with the provided socket address.
            // We also then start serving the web server, this will then block the application from running.
            // We also add a signal receiver that listens to a sender signal. Once that signal is received,
            // We can then unblock the application to shutdown gracefully.

            // Here we just wait for the blocked application to either receive a signal, or an error that requires the server to exit.
            // This allows us to atleast propergate the error the call stack.
            if let Err(_) = self.router.serve(socket_address).await {
                shutdown_signal.send(()).ok();
            };
        });

        Ok(())
    }
}
