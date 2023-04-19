use super::auth::StandardClaims;
use crate::{domain::system::error::error::SystemError, lib::database::database};
use axum::http::StatusCode;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use sqlx::{PgPool, Row};
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
    signing_method: Algorithm,
    db: PgPool,
) -> Result<String, SystemError> {
    // Load the correct encoding key based on the encoding algorithm provided.
    let (mut header, key) = match load_encoding_key(&key_id, signing_method) {
        Ok((header, key)) => (header, key),
        Err(err) => return Err(err),
    };

    // Create and define when the token is created, and expires.
    let issued_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let expires_at = issued_at + Duration::from_secs(15 * 60).as_secs();

    // Fetch the user from the database with the given user id to store in the claims.
    let query = "
        SELECT email, first_name, last_name, role
        FROM users
        WHERE id = $1";

    // Provide the statement.
    let statement = sqlx::query(query).bind(user_id);

    // Fetch a single row of a user by using fn query_single_row()
    let row = match database::query_single_row(&db, statement).await {
        Ok(rows) => rows,
        Err(err) => {
            return Err(SystemError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("could not locate user in database : {}", err.to_string()),
            ))
        }
    };

    // Allow the header to contain the key id.
    header.kid = Some(key_id);

    // Create out new standard claims object.
    let standard_claims = StandardClaims {
        email: row.get("email"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        role: row.get("role"),
        aud: String::from("external-api"),
        iss: String::from("external-api"),
        sub: String::from("10"), // we should get this from the user uuid in the db.
        iat: issued_at,
        exp: expires_at,
    };

    // We then run the encode function to create a new jwt using our private key.
    let new_token = match jsonwebtoken::encode(&header, &standard_claims, &key) {
        Ok(new_token) => new_token,
        Err(err) => {
            return Err(SystemError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "failed to encode new access token during encode : {}",
                    err.to_string()
                ),
            ))
        }
    };

    Ok(new_token)
}

// fn load_encoding_key() loads the correct encoding from the project based on the algorithm.
fn load_encoding_key(
    key_id: &str,
    signing_method: Algorithm,
) -> Result<(Header, EncodingKey), SystemError> {
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
                        "could not match path, found none",
                    ))
                }
            };

            // Use our private key based on the incoming key id.
            let private_key_name = format!("private-{}.pem", key_id);

            // We get the key location.
            let key_path = format!("{}/scaffold/keys/{}", abs_path, private_key_name);
            let mut key_file = match fs::File::open(key_path) {
                Ok(key_file) => key_file,
                Err(err) => {
                    return Err(SystemError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!(
                            "key location by key id does not exist : {}",
                            err.to_string()
                        ),
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
                        format!("could not read from buffer : {}", err.to_string()),
                    ))
                }
            };

            // We then load the encoding key using the correct functionality based on signing method.
            let key = match EncodingKey::from_rsa_pem(buf.as_bytes()) {
                Ok(key) => key,
                Err(err) => {
                    return Err(SystemError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!(
                            "could not create token with found encoding key : {}",
                            err.to_string()
                        ),
                    ));
                }
            };
            (alg, key)
        }
        _ => {
            return Err(SystemError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "invalid signing key method",
            ))
        }
    };

    Ok((header, key))
}
