use super::User;
use crate::business::core::user::V1PostUser;
use crate::foundation::database;
use crate::foundation::logger::logger::Logger;
use sqlx::error::UnexpectedNullError;
use sqlx::{PgPool, Row};

#[derive(Clone)]
pub struct UserStore {
    pub logger: Logger,
    pub db: PgPool,
}

// fn new_store() creates a new user store to perform database operations for the entity users.
pub fn new_store(logger: Logger, db: PgPool) -> UserStore {
    UserStore {
        logger: logger,
        db: db,
    }
}

// We only allow these functions to be accesible on the UserStore type.
// UserStore can have other store related packages within to further flavour our logic.
impl UserStore {
    // fn query_users() is the store function to query all users from the database.
    pub async fn query_users(&self) -> Result<Vec<User>, UnexpectedNullError> {
        // Create our raw query string.
        let query = "
        SELECT email, first_name, last_name, role
        FROM users
       ";

        // * QueryBuilder can be used here, instead to allow search filters.

        // Provide the statement.
        let statement = sqlx::query(query);

        // Log query to the console.
        self.logger
            .info_w("selecting all users... : query : ", Some(query));

        // Fetch all rows of users by using fn query_many_rows()
        let rows = database::database::query_many_rows(&self.db, statement)
            .await
            .unwrap_or_else(|err| {
                return Err(err).unwrap();
            });

        // Map the rows back into our concrete type Vector of User structs.
        let users = rows
            .iter()
            .map(|row| User {
                email: row.get("email"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                role: row.get("role"),
            })
            .collect();

        Ok(users)
    }

    pub async fn query_user_by_id(&self, id: i16) -> Result<User, sqlx::Error> {
        // Create our raw query string.
        let query = "
        SELECT email, first_name, last_name, role
        FROM users
        WHERE id = $1";

        // Provide the statement.
        let statement = sqlx::query(query).bind(id);

        // Log query to the console.
        self.logger
            .info_w("selecting user by id... : query : ", Some(query));

        // Fetch a single row of a user by using fn query_single_row()
        let row = database::database::query_single_row(&self.db, statement)
            .await
            .unwrap_or_else(|err| {
                return Err(err).unwrap();
            });

        // Map a single user struct to the returned rows given by the query.
        Ok(User {
            email: row.get("email"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            role: row.get("role"),
        })
    }

    pub async fn create_user(&self, user: V1PostUser) -> Result<(), sqlx::Error> {
        // Create our raw query string.
        let query = "
        INSERT INTO users(email, first_name, last_name, role)
        VALUES ($1,$2,$3,$4)";

        // Provide the statement.
        let statement = sqlx::query(query)
            .bind(user.email)
            .bind(user.first_name)
            .bind(user.last_name)
            .bind(user.role);

        // Log query to the console.
        self.logger
            .info_w("creating user... : query : ", Some(query));

        // Insert a new user record into the database using the mutate_statement()
        if let Err(err) = database::database::mutate_statement(&self.db, statement).await {
            return Err(err);
        }

        Ok(())
    }
}
