use serde::Deserialize;
use validator::Validate;

// Core Request Struct that acts as the payload for creating a new user.
#[derive(Deserialize, Validate)]
pub struct V1PostUser {
    #[validate(email)]
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
}
