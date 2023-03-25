// The version 1 routes for handling users.

use axum::response::IntoResponse;
use axum_macros::debug_handler;

pub struct UserHandlers {
    pub version: String,
}

impl UserHandlers {
    #[debug_handler]
    pub async fn get_users() -> impl IntoResponse {
        print!("hello!");
    }

    pub fn get_users_by_id(self) {}
}
