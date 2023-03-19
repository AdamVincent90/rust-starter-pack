use std::{thread, time::Duration};

use crate::foundation::logger::logger;
use actix_web::{dev::ServerHandle, get, rt, App, Error, HttpServer, Responder};
use awc::error::SendRequestError;

pub struct Config {
    pub web_address: String,
    pub port: u16,
}

pub async fn new_actix_server(config: Config) -> Result<ServerHandle, Error> {
    let srv = HttpServer::new(|| App::new().service(ping))
        .bind((config.web_address, config.port))
        .unwrap()
        .run();

    let handler = srv.handle();
    rt::spawn(srv);

    Ok(handler)
}

#[get("/")]
async fn ping() -> impl Responder {
    format!("ping successful")
}

pub async fn ping_axtix_server(
    log: &logger::Logger,
    max_attempts: u8,
) -> Result<(), SendRequestError> {
    thread::sleep(Duration::from_secs(5));
    let client = awc::Client::default();

    for i in 1..=max_attempts {
        match client.get("http://localhost:6874").send().await {
            Ok(res) => {
                log.info_w("server successfuly pinged", Some(res));
                break;
            }
            Err(err) => {
                if i == max_attempts {
                    log.error_w("failed to ping web server", Some(&err));
                    return Err(err);
                }
            }
        };
    }

    Ok(())
}
