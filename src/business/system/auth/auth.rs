use super::{decode, encode::encode_token};
use crate::business::{
    core::user::stores::user_db::user_db::UserStore, web::state::shared::MuxState,
};
use hyper::StatusCode;
use jsonwebtoken::{self, Algorithm};
use serde::{Deserialize, Serialize};
use tokio::sync::{RwLockReadGuard, RwLockWriteGuard};

#[derive(Clone)]
// The main auth struct that will be used to authenticate, and authorise a user.
pub struct Auth {
    pub enabled: bool,
    pub key_id: String,
    pub signing_method: Algorithm,
    pub user_store: UserStore,
}

// The configuration when creating a new auth instance.
pub struct AuthConfig {
    pub enabled: bool,
    pub key_id: String,
    pub signing_method: Algorithm,
    pub user_store: UserStore,
}

// The struct that contains all standard claims common within a JWT.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

pub fn new(config: AuthConfig) -> Auth {
    Auth {
        enabled: config.enabled,
        key_id: config.key_id,
        signing_method: config.signing_method,
        user_store: config.user_store,
    }
}

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
    pub fn authenticate(
        &self,
        token: String,
        mutex: &mut RwLockWriteGuard<MuxState>,
    ) -> Result<(), StatusCode> {
        let data = match decode::validate_token(token, &self.key_id, self.signing_method) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        // Save to claims from our mutex shared state.
        mutex.claims = data.claims;

        Ok(())
    }

    // pub fn authorise() checks the claims to verify if they contain the information we would like them to contain.
    pub fn authorise(
        &self,
        mutex: &RwLockReadGuard<MuxState>,
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

        if !list.contains(&mutex.claims.role) {
            return Err(StatusCode::UNAUTHORIZED);
        }

        Ok(())
    }
}
