use std::io::Error;

use rust_starter_pack::dependency::logger::logger;
use serde::{de::DeserializeOwned, Deserialize};

// Every main.rs executable, in most cases, should have a config for the app, these configs
// aim to provide the app context to where or what they are performing business logic to, or for.

// ################################################
// Add any custom structs that derive Deserialize below, make sure your property names match your env vars.
// Note you can provide prefixs in main.rs, so you property names dont require underscores.

// We derive Deserialize to allow env vars to map to this struct.
#[derive(Deserialize)]
pub struct AppSettings {
    pub version: String,
    pub environment: String,
}

// We derive Deserialize to allow env vars to map to this struct.
#[derive(Deserialize)]
pub struct WebSettings {
    pub address: String,
    pub port: u16,
    pub debug_address: String,
    pub debug_port: u16,
}

// We derive Deserialize to allow env vars to map to this struct.
#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub schema: String,
}

#[derive(Deserialize)]
pub struct AuthSettings {
    pub key_id: String,
}

// I want to derive these :(
// But this allow our custom setting structs to implement the Conf trait and to have access to the load_from_env() default function.
// For now, please implement the Conf trait for your custom struct. Once done, you can add your defaults to main.rs, and they will
// be overridden by any valid values from .env.
impl Conf for AppSettings {}
impl Conf for WebSettings {}
impl Conf for DatabaseSettings {}
impl Conf for AuthSettings {}

// ################################################

// Ok, so i found a neat way to have a singular trait that contains a default function to load and map envs
// from the .env file, every setting struct made just needs to derive Deserialize interface to satisfy the
// envy function that requires a struct that implements Deserialize.
// We then make a new trait that can only be implemented by types derviving Deserialize, this then allows
// This trait default function to map envs correctly, without any changes to the functionality itself,
// also allowing default values assigned in main.rs to be used if a mapping was not found.
pub trait Conf: DeserializeOwned {
    fn load_from_env(self, logger: &logger::Logger, prefix: &str) -> Result<Self, Error> {
        // Firstly, we check if dotenv() returns the Err result, if it does, we can log a warning that
        // it failed to load then instead of returning the error, we can simply return the default
        // Configuration for this particular struct.
        if let Err(err) = dotenvy::dotenv() {
            logger.warn_w(
                format!(
                    "failed to load .env file, reverting to default : reason {}",
                    err.to_string()
                )
                .as_str(),
                Some("Rust Web API Start Up"),
            );
            return Ok(self);
        }

        // We can provide a prefix to this function, that will be appended to get environment vars with this prefix
        // For example; prefix = "DB" | DatabaseSettings {host: "localhost"} | .env = DB_HOST
        let mut env_prefix = String::new();
        if prefix != "" {
            env_prefix = format!("{}_", prefix.to_uppercase());
        }

        // We use the prefix to then find any env vars loaded from dotenv with the prefix, because this a a trait
        // that can only be implemented by types with Derserialize, we can then cast Self to return the type
        // with the mapped values. If these values cannot be found, we simply warn and return the default value.
        let loaded_env = envy::prefixed(env_prefix)
            .from_env::<Self>()
            .unwrap_or_else(|err| {
                logger.warn_w(
                    format!(
                        "Could not load settings from env. reverting to default : reason {}",
                        err.to_string()
                    )
                    .as_str(),
                    Some("Rust Web API Main Config"),
                );
                return self;
            });

        Ok(loaded_env)
    }
}
