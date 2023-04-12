use log::LevelFilter;
use openssl::rsa::Rsa;
use rust_starter_pack::dependency::logger::logger;
use rust_starter_pack::dependency::logger::logger::Logger;
use std::env;
use std::fs::{create_dir_all, write};
use std::path::PathBuf;

const RSA_BITS: u32 = 4096;

fn main() {
    env::set_var("RUST_LOG", "info");
    let logger = logger::new_logger(logger::Config {
        name: String::from("OPENSSL-GEN"),
        max_log_level: LevelFilter::Info,
    });

    logger.info_w("starting open-ssl key generation", Some("SSL main"));

    let uuid = match run(&logger) {
        Ok(uuid) => uuid,
        Err(err) => {
            logger.error_w(
                format!("error during run process : {}", err.to_string()).as_str(),
                Some("SSL main"),
            );
            std::process::exit(1);
        }
    };

    logger.warn_w(
        "key pair generated and available in scaffold/certs, its unwise to share the private key.",
        Some("SSL main"),
    );

    logger.warn_w(
        format!(
            "your key [{}] can be saved in your .env, and is used to identify your key",
            uuid
        )
        .as_str(),
        Some("SSL main"),
    )
}

fn run(logger: &Logger) -> Result<String, Box<dyn std::error::Error>> {
    logger.info_w(
        format!("generating RSA with {} bits", RSA_BITS).as_str(),
        Some("SSL run"),
    );

    let rsa = match Rsa::generate(RSA_BITS) {
        Ok(rsa) => rsa,
        Err(err) => return Err(Box::new(err)),
    };

    let rsa = match openssl::pkey::PKey::from_rsa(rsa) {
        Ok(rsa) => rsa,
        Err(err) => return Err(Box::new(err)),
    };

    let private_key = match rsa.private_key_to_pem_pkcs8() {
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

    // Create a random uuid that acts as the unique identifier and key lookup for certain auth systems.
    let uuid = uuid::Uuid::new_v4().to_string();

    let private_key_name = format!("private-{}.pem", uuid);
    let public_key_name = format!("public-{}.pem", uuid);

    // Write the new file to path.
    if let Err(err) = write(format!("{}/{}", cert_path, private_key_name), private_key) {
        return Err(Box::new(err));
    }

    let public_key = match String::from_utf8(public_key) {
        Ok(public_key) => public_key,
        Err(err) => {
            return Err(Box::new(err));
        }
    };

    // Write the new file to path.
    if let Err(err) = write(format!("{}/{}", cert_path, public_key_name), public_key) {
        return Err(Box::new(err));
    }

    Ok(uuid)
}
