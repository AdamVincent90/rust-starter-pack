use std::{thread, time::Duration};

use crate::foundation::logger::logger;
use actix_web::{dev::ServerHandle, get, rt, App, Error, HttpServer, Responder};
use awc::error::SendRequestError;

// To clean up and improve

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

    // This adds the server to a new thread so the application is not awaiting returning handler.
    // This whole function will need lots of tidy up.
    rt::spawn(srv);

    Ok(handler)
}

#[get("/")]
async fn ping() -> impl Responder {
    format!("ping successful")
}

pub async fn ping_actix_server(
    log: &logger::Logger,
    max_attempts: u8,
) -> Result<(), SendRequestError> {
    thread::sleep(Duration::from_secs(5));
    let client = awc::Client::default();

    for i in 1..=max_attempts {
        match client.get("http://localhost:80").send().await {
            Ok(res) => {
                log.info_w("actix server successfuly pinged", Some(res.status()));
                break;
            }
            Err(err) => {
                if i == max_attempts {
                    log.error_w("failed to ping actix server", Some(&err));
                    return Err(err);
                }
            }
        };
    }

    log.info_w("actix ping operation completed", Some(()));

    Ok(())
}
