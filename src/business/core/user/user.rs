use super::{
    stores::user_db::{
        user_db::{self, UserStore},
        User,
    },
    V1PostUser,
};
use crate::dependency::logger::logger::Logger;
use sqlx::{error::UnexpectedNullError, PgPool};

#[derive(Clone)]
pub struct UserCore {
    user_store: UserStore,
}

// fn new_core() constructs a new core to perform core business logic for users.
pub fn new_core(logger: &Logger, db: &PgPool) -> UserCore {
    UserCore {
        user_store: user_db::new_store(logger.clone(), db.clone()),
    }
}

// We only allow these functions to be accesible on the UserCore type.
// User core currently supports UserStore, but as with core functionality
// There may be an abundance of core packages that can be used,
// One example can be business/core/user/clients/[grpc, rest] that will allow this core to send requests.
impl UserCore {
    // fn v1_get_users() is the core entrypoint to start user business logic for getting all users.
    pub async fn v1_get_users(&self) -> Result<User, UnexpectedNullError> {
        let result = self
            .user_store
            .query_user_by_id(1)
            .await
            .unwrap_or_else(|err| {
                return Err(err).unwrap();
            });

        Ok(result)
    }
    // fn v1_get_users_by_id() is the core entrypoint to start user business logic for getting a user by id.
    pub async fn v1_get_users_by_id(&self) -> Result<User, UnexpectedNullError> {
        let result = self
            .user_store
            .query_user_by_id(1)
            .await
            .unwrap_or_else(|err| {
                return Err(err).unwrap();
            });

        Ok(result)
    }
    // fn v1_post_user() is the core entrypoint to start user business logic for creating a new user.
    pub async fn v1_post_user(&self, user: V1PostUser) -> Result<(), UnexpectedNullError> {
        self.user_store
            .create_user(user)
            .await
            .unwrap_or_else(|err| {
                return Err(err).unwrap();
            });

        Ok(())
    }
}
