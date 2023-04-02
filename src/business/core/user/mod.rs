pub mod user;
pub mod stores {
    pub mod user_db;
}

use serde::Deserialize;
use validator::Validate;
#[derive(Deserialize, Validate)]
pub struct V1PostUser {
    #[validate(email)]
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
}
