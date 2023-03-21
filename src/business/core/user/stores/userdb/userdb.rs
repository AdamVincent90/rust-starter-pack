// This is where all logic goes to perform user based database operations.

use sqlx::error::UnexpectedNullError;
use sqlx::{postgres, FromRow, Row};

pub struct User {
    email: String,
    first_name: String,
    last_name: String,
    role: String,
}

impl FromRow<'_, postgres::PgRow> for User {
    fn from_row(row: &postgres::PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            email: row.try_get("email")?,
            first_name: row.try_get("first_name")?,
            last_name: row.try_get("last_name")?,
            role: row.try_get("role")?,
        })
    }
}

pub fn query_user_by_id(id: u16) -> Result<User, UnexpectedNullError> {
    match id {
        0 => Err(UnexpectedNullError),
        _ => Ok(User {
            email: String::from("john.doe@example.com"),
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            role: String::from("user"),
        }),
    }
}
