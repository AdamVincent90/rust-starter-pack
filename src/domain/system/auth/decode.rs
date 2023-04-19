use super::auth::StandardClaims;
use crate::domain::system::error::error::SystemError;
use axum::http::StatusCode;
use jsonwebtoken::{self, Algorithm, DecodingKey, TokenData, Validation};
use std::{env, fs, io::Read, path::PathBuf};

/// Decode abstracts away the logic that performs the decoding, and validation of a JWT.

// pub fn validate_token() checks the signing method and token, to perform the correct decode procedure.
// If the JWT is not valid, or the public key is incorrect, then we simply return an error.
pub fn validate_token(
    token: String,
    key_id: &String,
    signing_method: Algorithm,
) -> Result<TokenData<StandardClaims>, SystemError> {
    // We obtain the relevant decoding key (private.pem for RSA256 etc)
    let key = match load_decoding_key(&key_id, signing_method) {
        Ok(key) => key,
        Err(err) => return Err(err),
    };

    // We then use that decoding key on the incoming token to validate its legitimacy, if so, then we map the token
    // to the claims.
    let data: TokenData<StandardClaims> =
        match jsonwebtoken::decode(&token, &key, &Validation::new(signing_method)) {
            Ok(data) => data,
            Err(err) => return Err(SystemError::new(StatusCode::UNAUTHORIZED, err.to_string())),
        };

    Ok(data)
}

// fn load_decoding_key() loads the correct public key or secret based on the signing method passed in.
fn load_decoding_key(key_id: &str, signing_method: Algorithm) -> Result<DecodingKey, SystemError> {
    // Based on the signing method, we load a different key for our project.
    let key = match signing_method {
        Algorithm::HS256 => DecodingKey::from_secret("secret".as_bytes()),
        Algorithm::RS256 => {
            // We get the absolute path.
            let abs_path = PathBuf::from(match env::current_dir() {
                Ok(abs_path) => abs_path,
                Err(err) => {
                    return Err(SystemError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("could not locate absolute path : {}", err.to_string()),
                    ));
                }
            });

            let abs_path = match abs_path.to_str() {
                Some(abs_path) => abs_path,
                None => {
                    return Err(SystemError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "not match path absolute path",
                    ))
                }
            };

            let public_key_name = format!("public-{}.pem", key_id);

            // We get the key location.
            let key_path = format!("{}/scaffold/keys/{}", abs_path, public_key_name);
            let mut key_file = match fs::File::open(key_path) {
                Ok(key_file) => key_file,
                Err(err) => {
                    return Err(SystemError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("could not find key file by key id : {}", err.to_string()),
                    ))
                }
            };

            // We create a buffer for our string to buffer into bytes.
            let mut buf = String::new();
            match key_file.read_to_string(&mut buf) {
                Ok(buf) => buf,
                Err(err) => {
                    return Err(SystemError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("failed to read buffer : {}", err.to_string()),
                    ))
                }
            };

            // We then load the decoding key using the correct functionality based on signing method.
            let key = match DecodingKey::from_rsa_pem(buf.as_bytes()) {
                Ok(key) => key,
                Err(err) => {
                    return Err(SystemError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("invalid access token : {}", err.to_string()),
                    ));
                }
            };
            key
        }
        _ => {
            return Err(SystemError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "unsupported signing method",
            ))
        }
    };

    Ok(key)
}
