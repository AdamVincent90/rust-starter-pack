// This is where all logic goes to perform user based database operations.

use sqlx::error::UnexpectedNullError;
use sqlx::{PgPool, QueryBuilder};

use crate::business::core::user::models::V1PostUser;
use crate::foundation::database;
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

    pub async fn create_user(&self, user: V1PostUser) -> Result<(), sqlx::Error> {
        println!("in store!");

        println!("{}", user.email);

        // Absolutley disgusting, may need to create a wrapper, or investigate other methods
        // that allows functions to take in structs or then map/serialise according
        let mut query = QueryBuilder::new(
            "INSERT INTO users(email, first_name, last_name, role) VALUES ($1,$2,$3,$4)",
        );
        query.push_bind(user.email);
        query.push_bind(user.first_name);
        query.push_bind(user.last_name);
        query.push_bind(user.role);

        if let Err(err) = database::database::execute_statement(&self.db, query.sql()).await {
            return Err(err);
        }

        Ok(())
    }
}
