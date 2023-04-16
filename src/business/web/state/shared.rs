// State contains all the the context, state, and extensions for business logic to have access
// to desired state.

use crate::business::system::auth::auth::StandardClaims;
use std::sync::Arc;
use tokio::sync::RwLock;

// The state that is shared across services.
pub type SharedState = Arc<RwLock<MuxState>>;

// Any data that would be advantagous to contain for shared readable, and writable state.
pub struct MuxState {
    pub claims: StandardClaims,
}
