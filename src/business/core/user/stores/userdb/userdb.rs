// This is where all logic goes to perform user based database operations.

use sqlx::error::UnexpectedNullError;
use sqlx::PgPool;

use crate::foundation::logger::logger::Logger;

use super::models::User;

#[derive(Clone)]
pub struct UserStore {
    pub logger: Logger,
    pub db: PgPool,
}

pub fn new_store(logger: Logger, db: PgPool) -> UserStore {
    UserStore {
        logger: logger,
        db: db,
    }
}

impl UserStore {
    pub fn query_users(&self) -> Result<User, UnexpectedNullError> {
        println!("in store!");
        Ok(User {
            email: String::from("john.doe@example.com"),
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            role: String::from("user"),
        })
    }
    pub fn query_user_by_id(&self, id: u16) -> Result<User, UnexpectedNullError> {
        println!("in store!");
        match id {
            0 => Err(UnexpectedNullError),
            _ => {
                let user = User {
                    email: String::from("john.doe@example.com"),
                    first_name: String::from("John"),
                    last_name: String::from("Doe"),
                    role: String::from("user"),
                };
                Ok(user)
            }
        }
    }
}
