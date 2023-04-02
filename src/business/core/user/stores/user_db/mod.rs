pub mod user_db;

// * mod.rs makes sense to also contain the models for the module.
use serde::Serialize;
// Store Struct that represents the User, as is stored in the database.
#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
}
