use serde::Serialize;

// * Your store models should best represent the entity as you select it from a database.

// Store Struct that represents the User, as is stored in the database.
#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
}
