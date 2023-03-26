// The version 1 routes for handling users.

use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use ultimate_rust_service::{business, foundation::server::server::Axum};

#[derive(Debug, Serialize, Deserialize)]
struct UserResponse {
    message: String,
}

// adding the traits as super traits here means you don't have to specify them
// in each `where` clause
trait Handler: Clone + Send + 'static {
    fn v1_get_users(self) -> Result<String, String>;
    fn v1_get_users_by_id(self) -> Result<String, String>;
}

#[derive(Clone)]
pub struct UserHandlers {
    pub version: String,
    pub user_core: business::core::user::user::Core,
}

impl UserHandlers {
    pub fn register_user_routes(self, router: &Router, version: &str) {
        // router
        //     // GET ( /v1/users )
        //     .route(
        //         format!("{}{}", version, "/users").as_str(),
        //         get(|| async { self.v1_get_users() }),
        //     )
        //     // GET ( /v1/users/:id )
        //     .route(
        //         format!("{}{}", version, "/users/:id").as_str(),
        //         get(|| async { self.v1_get_users_by_id() }),
        //     );
    }
}

impl Handler for UserHandlers {
    fn v1_get_users(self) -> Result<String, String> {
        Ok(String::from("value"))
    }

    fn v1_get_users_by_id(self) -> Result<String, String> {
        Ok(String::from("value with id"))
    }
}
