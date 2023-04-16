use super::{
    stores::user_db::{
        user_db::{self, UserStore},
        User,
    },
    V1PostUser,
};
use crate::business::{system::error::error::RequestError, web::state::shared::MuxState};
use crate::dependency::logger::logger::Logger;
use sqlx::PgPool;

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
    pub async fn get_all(&self, state: &MuxState) -> Result<Vec<User>, RequestError> {
        println!(
            "You are authed as {} with the role {}",
            state.claims.email, state.claims.role
        );

        let result = match self.user_store.query_users().await {
            Ok(result) => result,
            Err(err) => {
                return Err(RequestError::new(
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    err.to_string(),
                ));
            }
        };

        Ok(result)
    }
    // fn v1_get_users_by_id() is the core entrypoint to start user business logic for getting a user by id.
    pub async fn get_by_id(&self, state: &MuxState, id: i32) -> Result<User, RequestError> {
        println!(
            "You are authed as {} with the role {}",
            state.claims.email, state.claims.role
        );
        let result = match self.user_store.query_user_by_id(id).await {
            Ok(result) => result,
            Err(err) => {
                return Err(RequestError::new(
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    err.to_string(),
                ));
            }
        };
        Ok(result)
    }
    // fn v1_post_user() is the core entrypoint to start user business logic for creating a new user.
    pub async fn create(&self, state: &MuxState, user: V1PostUser) -> Result<(), RequestError> {
        println!(
            "You are authed as {} with the role {}",
            state.claims.email, state.claims.role
        );
        if let Err(err) = self.user_store.create_user(user).await {
            return Err(RequestError::new(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                err.to_string(),
            ));
        }

        Ok(())
    }
}
