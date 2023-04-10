use std::{env, fs, io::Read, path::PathBuf};

use jsonwebtoken::{self, Algorithm, DecodingKey, TokenData, Validation};

use super::auth::StandardClaims;

/// Decode abstracts away the logic that performs the decod, and validation of a JWT.

// pub fn validate_token() checks the signing method and token, to perform the correct decode procedure.
// If the JWT is not valid, or the public key is incorrect, then we simply return an error.
pub fn validate_token(
    token: String,
    signing_method: Algorithm,
) -> Result<TokenData<StandardClaims>, axum::http::StatusCode> {
    // We obtain the relevant decoding key (private.pem for RSA256 etc)
    let key = match load_decoding_key(signing_method) {
        Ok(key) => key,
        Err(err) => return Err(err),
    };

    // We then use that decoding key on the incoming token to validate its legitimacy, if so, then we map the token
    // to the claims.
    let data: TokenData<StandardClaims> =
        match jsonwebtoken::decode(&token, &key, &Validation::new(Algorithm::RS256)) {
            Ok(data) => data,
            Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        };

    Ok(data)
}

// fn load_decoding_key() loads the correct public key or secret based on the signing method passed in.
fn load_decoding_key(signing_method: Algorithm) -> Result<DecodingKey, hyper::StatusCode> {
    // Based on the signing method, we load a different key for our project.
    let key = match signing_method {
        Algorithm::HS256 => DecodingKey::from_secret("secret".as_bytes()),
        Algorithm::RS256 => {
            // We get the absolute path.
            let abs_path = PathBuf::from(match env::current_dir() {
                Ok(abs_path) => abs_path,
                Err(_) => {
                    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
                }
            });

            let abs_path = match abs_path.to_str() {
                Some(abs_path) => abs_path,
                None => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
            };

            // We get the key location.
            let key_path = format!("{}/scaffold/certs/public.pem", abs_path);
            let mut key_file = match fs::File::open(key_path) {
                Ok(key_file) => key_file,
                Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
            };

            // We create a buffer for our string to buffer into bytes.
            let mut buf = String::new();
            match key_file.read_to_string(&mut buf) {
                Ok(buf) => buf,
                Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
            };

            // We then load the decoding key using the correct functionality based on signing method.
            let key = match DecodingKey::from_rsa_pem(buf.as_bytes()) {
                Ok(key) => key,
                Err(_) => {
                    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
                }
            };
            key
        }
        _ => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(key)
}
