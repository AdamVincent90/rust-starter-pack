// This will be the core business logic to handle a user.

use sqlx::{error::UnexpectedNullError, PgPool};

use crate::foundation::logger::logger::Logger;

use super::{
    models::V1PostUser,
    stores::userdb::{
        models::User,
        userdb::{self, UserStore},
    },
};

#[derive(Clone)]
pub struct Core {
    user_store: UserStore,
}

pub fn new_core(logger: &Logger, db: &PgPool) -> Core {
    Core {
        user_store: userdb::new_store(logger.clone(), db.clone()),
    }
}

impl Core {
    pub fn v1_get_users(&self) -> Result<User, UnexpectedNullError> {
        println!("in core!");
        let result = self.user_store.query_user_by_id(1).unwrap_or_else(|err| {
            return Err(err).unwrap();
        });

        Ok(result)
    }

    pub fn v1_get_users_by_id(&self) -> Result<User, UnexpectedNullError> {
        println!("in core!");
        let result = self.user_store.query_user_by_id(1).unwrap_or_else(|err| {
            return Err(err).unwrap();
        });

        Ok(result)
    }

    pub async fn v1_post_user(&self, user: V1PostUser) -> Result<(), UnexpectedNullError> {
        println!("in core!");
        self.user_store
            .create_user(user)
            .await
            .unwrap_or_else(|err| {
                return Err(err).unwrap();
            });

        Ok(())
    }
}
