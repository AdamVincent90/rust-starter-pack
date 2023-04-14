// My own rule.
// Extensions = Shared state, for example auth where we want to use the same state and one source of truth.
// Context = Isolated state, for example handler contexts that have their own implementation of state.

use crate::business::system::auth::auth::Auth;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type WebState = Arc<RwLock<MuxState>>;

pub struct MuxState {
    pub auth: Auth,
}
