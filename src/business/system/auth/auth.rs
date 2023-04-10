// Remove when no longer required.
#![allow(dead_code, unused)]
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{self, Algorithm, EncodingKey};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};

use crate::{business::core::user::stores::user_db::User, dependency::database};

// TODO - create this.

// The main auth struct that will be used to authenticate, and authorise a user.
pub struct Auth {
    pub claims: StandardClaims,
    signing_method: Algorithm,
    token: String,
}

// The configuration when creating a new auth instance.
pub struct AuthConfig {
    pub signing_method: Algorithm,
}

// The struct that contains all standard claims common within a JWT.
#[derive(Debug, Serialize, Deserialize)]
pub struct StandardClaims {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub aud: String,
    pub exp: u64,
    pub iat: u64,
    pub iss: String,
    pub sub: String,
}

// Implement Auth based functionality.
impl Auth {
    pub fn authenticate(&self, token: String) {
        todo!()
    }

    pub fn authorise(&self, claims: StandardClaims) {
        todo!()
    }

    async fn create_token(
        &self,
        user_id: i32,
        db: PgPool,
    ) -> Result<String, axum::http::StatusCode> {
        let (header, key) = match self.signing_method {
            Algorithm::HS256 => {
                let mut alg = jsonwebtoken::Header::default();
                alg.alg = Algorithm::HS256;
                let key = EncodingKey::from_secret("secret".as_bytes());
                (alg, key)
            }
            Algorithm::RS256 => {
                let mut alg = jsonwebtoken::Header::default();
                alg.alg = Algorithm::RS256;
                let key = match EncodingKey::from_rsa_pem("pem".as_bytes()) {
                    Ok(key) => key,
                    Err(_) => {
                        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
                    }
                };
                (alg, key)
            }
            _ => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        };

        let issued_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let expires_at = issued_at + Duration::from_secs(15 * 60).as_secs();

        let user = match get_user_by_id(user_id, db).await {
            Ok(user) => user,
            Err(_) => {
                return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

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

        let new_token = match jsonwebtoken::encode(&header, &standard_claims, &key) {
            Ok(new_token) => new_token,
            Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        };

        Ok(new_token)
    }

    fn validate(&self, token: String) -> Result<bool, axum::http::StatusCode> {
        // Is accepted by pub key
        // exp has not been exceeded (refresh token?)
        // user claims in database
        todo!()
    }
}

// Use the user store.
async fn get_user_by_id(id: i32, db: PgPool) -> Result<User, sqlx::Error> {
    // Create our raw query string.
    let query = "
        SELECT email, first_name, last_name, role
        FROM users
        WHERE id = $1";

    // Provide the statement.
    let statement = sqlx::query(query).bind(id);

    // Fetch a single row of a user by using fn query_single_row()
    let row = match database::database::query_single_row(&db, statement).await {
        Ok(rows) => rows,
        Err(err) => return Err(err),
    };

    // Map a single user struct to the returned rows given by the query.
    Ok(User {
        email: row.get("email"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        role: row.get("role"),
    })
}
