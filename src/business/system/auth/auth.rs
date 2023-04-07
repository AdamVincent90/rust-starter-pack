use jsonwebtoken::{self, Algorithm};
use serde::{Deserialize, Serialize};

// TODO - create this.

// The main auth struct that will be used to authenticate, and authorise a user.
pub struct Auth {
    pub user: UserClaims,
    pub claims: StandardClaims,
}

// The configuration when creating a new auth instance.
pub struct AuthConfig {
    pub signing_method: Algorithm,
}

// The struct that contains all user based claims that can be used to further authorise a user.
pub struct UserClaims {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
}

// The struct that contains all standard claims common within a JWT.
#[derive(Debug, Serialize, Deserialize)]
pub struct StandardClaims {
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub nbf: usize,
    pub sub: String,
}

// Create a new instance of Auth to perform authentication, and authorisation.
pub fn new(config: AuthConfig) {
    match config.signing_method {
        Algorithm::HS256 => {
            // Some logic
        }
        Algorithm::RS256 => {
            // Some logic
        }
        _ => {
            // Unsupported signing method.
        }
    }
}

// Implement Auth based functionality.
impl Auth {
    pub fn authenticate(&self) {}

    pub fn authorise(&self) {}
}
