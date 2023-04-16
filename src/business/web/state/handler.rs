use crate::business;

// UserContext contains any state required when it comes to working with user operations.
#[derive(Clone)]
pub struct UserContext {
    pub version: String,
    pub user_core: business::core::user::user::UserCore,
}
