use std::env;
use std::fs::{create_dir_all, write};
use std::path::PathBuf;

use log::LevelFilter;
use openssl::rsa::Rsa;
use rust_starter_pack::dependency::logger::logger;
use rust_starter_pack::dependency::logger::logger::Logger;

const RSA_BITS: u32 = 4096;

fn main() {
    env::set_var("RUST_LOG", "info");
    let logger = logger::new_logger(logger::Config {
        name: String::from("OPENSSL-GEN"),
        max_log_level: LevelFilter::Info,
    });

    logger.info_w("starting open-ssl key generation", Some("SSL main"));

    if let Err(err) = run(&logger) {
        logger.error_w(
            format!("error during run process : {}", err.to_string()).as_str(),
            Some("SSL main"),
        )
    }

    logger.warn_w(
        "key pair generated and available in scaffold/certs, its unwise to share the private key.",
        Some("SSL main"),
    )
}

fn run(logger: &Logger) -> Result<(), Box<dyn std::error::Error>> {
    logger.info_w(
        format!("generating RSA with {} bits", RSA_BITS).as_str(),
        Some("SSL run"),
    );

    let rsa = match Rsa::generate(RSA_BITS) {
        Ok(rsa) => rsa,
        Err(err) => return Err(Box::new(err)),
    };

    let private_key = match rsa.private_key_to_pem() {
        Ok(private_key) => private_key,
        Err(err) => return Err(Box::new(err)),
    };

    let public_key = match rsa.public_key_to_pem() {
        Ok(public_key) => public_key,
        Err(err) => return Err(Box::new(err)),
    };

    logger.info_w("private and public key generated", Some("SSL run"));

    // Store private key in scaffold/certs/key.pem
    // Define the absolute path.
    let abs_path = PathBuf::from(match env::current_dir() {
        Ok(abs_path) => abs_path,
        Err(err) => {
            return Err(Box::new(err));
        }
    });

    let abs_path = match abs_path.to_str() {
        Some(abs_path) => abs_path,
        None => return Err("could not convert absolute path to string".into()),
    };

    let cert_path = format!("{}/scaffold/certs", abs_path);

    if let Err(err) = create_dir_all(&cert_path) {
        return Err(Box::new(err));
    }

    let private_key = match String::from_utf8(private_key) {
        Ok(private_key) => private_key,
        Err(err) => {
            return Err(Box::new(err));
        }
    };

    if let Err(err) = write(format!("{}/private.pem", cert_path), private_key) {
        return Err(Box::new(err));
    }

    let public_key = match String::from_utf8(public_key) {
        Ok(public_key) => public_key,
        Err(err) => {
            return Err(Box::new(err));
        }
    };

    if let Err(err) = write(format!("{}/public.pem", cert_path), public_key) {
        return Err(Box::new(err));
    }

    Ok(())
}
