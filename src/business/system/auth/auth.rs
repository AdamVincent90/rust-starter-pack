use super::{decode, encode::encode_token};
use crate::business::core::user::stores::user_db::user_db::UserStore;
use hyper::StatusCode;
use jsonwebtoken::{self, Algorithm, TokenData};
use serde::{Deserialize, Serialize};

// The main auth struct that will be used to authenticate, and authorise a user.
pub struct Auth {
    pub key_id: String,
    pub signing_method: Algorithm,
    pub user_store: UserStore,
    pub claims: StandardClaims,
}

// The configuration when creating a new auth instance.
pub struct AuthConfig {
    pub key_id: String,
    pub signing_method: Algorithm,
    pub user_store: UserStore,
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

// Still a work in progress.

impl Auth {
    // Creates a new JWT for the given user id (will be uuid). Used either to manually create a token
    // Or to return a new token on successful login.
    pub async fn new_token(&self, user_id: i32) -> Result<String, StatusCode> {
        let data = match encode_token(
            user_id,
            self.key_id.clone(),
            self.user_store.clone(),
            self.signing_method,
        )
        .await
        {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        Ok(data)
    }

    // pub fn authenticate() Decodes and validates the incoming token, and if successful, maps and returns the claims.
    pub fn authenticate(&self, token: String) -> Result<TokenData<StandardClaims>, StatusCode> {
        let data = match decode::validate_token(token, self.signing_method) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        // Perform more checks

        Ok(data)
    }

    // pub fn authorise() checks the claims to verify if they contain the information we would like them to contain.
    pub fn authorise(
        &self,
        claims: &StandardClaims,
        roles: Option<Vec<String>>,
    ) -> Result<(), StatusCode> {
        // Very Basic for now.

        let list = match roles {
            Some(list) => list,
            None => {
                // Default roles here
                vec![String::from("admin")]
            }
        };

        if !list.contains(&claims.role) {
            return Err(StatusCode::UNAUTHORIZED);
        }

        Ok(())
    }
}
