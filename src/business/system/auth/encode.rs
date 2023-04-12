use super::auth::StandardClaims;
use crate::business::core::user::stores::user_db::user_db::UserStore;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use std::{
    env, fs,
    io::Read,
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

// pub async fn encode_token() creates a new token based on the encoding method passed.
pub async fn encode_token(
    user_id: i32,
    key_id: String,
    store: UserStore,
    signing_method: Algorithm,
) -> Result<String, axum::http::StatusCode> {
    // Load the correct encoding key based on the encoding algorithm provided.
    let (mut header, key) = match load_encoding_key(signing_method) {
        Ok((header, key)) => (header, key),
        Err(err) => return Err(err),
    };

    // Ass the key id to the token.
    header.kid = Some(key_id);

    // Create and define when the token is created, and expires.
    let issued_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let expires_at = issued_at + Duration::from_secs(15 * 60).as_secs();

    // Fetch the user from the database with the given user id to store in the claims.
    let user = match store.query_user_by_id(user_id).await {
        Ok(user) => user,
        Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Create JWT with unique key id etc.

    // Create out new standard claims object.
    let standard_claims = StandardClaims {
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        role: user.role,
        aud: String::from("rust-web-api"),
        iss: String::from("rust-web-api"),
        sub: String::from("10"), // we should get this from the user uuid in the db.
        iat: issued_at,
        exp: expires_at,
    };

    // We then run the encode function to create a new jwt using our private key.
    let new_token = match jsonwebtoken::encode(&header, &standard_claims, &key) {
        Ok(new_token) => new_token,
        Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(new_token)
}

// fn load_encoding_key() loads the correct encoding from the project based on the algorithm.
fn load_encoding_key(
    signing_method: Algorithm,
) -> Result<(Header, EncodingKey), hyper::StatusCode> {
    // Based on the signing method, we load a different key for our project.
    let (header, key) = match signing_method {
        Algorithm::HS256 => {
            let mut alg = jsonwebtoken::Header::default();
            alg.alg = Algorithm::HS256;
            let key = EncodingKey::from_secret("secret".as_bytes());
            (alg, key)
        }
        Algorithm::RS256 => {
            let mut alg = jsonwebtoken::Header::default();
            alg.alg = Algorithm::RS256;

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
            let key_path = format!("{}/scaffold/certs/private.pem", abs_path);
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

            // We then load the encoding key using the correct functionality based on signing method.
            let key = match EncodingKey::from_rsa_pem(buf.as_bytes()) {
                Ok(key) => key,
                Err(_) => {
                    return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
                }
            };
            (alg, key)
        }
        _ => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok((header, key))
}
