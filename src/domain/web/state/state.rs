// State contains all the the shared state available for the web service.

use crate::domain::system::auth::auth::StandardClaims;
use std::sync::Arc;
use tokio::sync::RwLock;

// The state that is shared across services.
// Here I decided to use RWLock instead of a Mutex, as there will be cases where we would like to write
// to our state. A very good use-case of this is our standard claims, that contains user information.
pub type SharedState = Arc<RwLock<MuxState>>;

// Any data that would be advantagous to contain for shared readable, and writable state.
pub struct MuxState {
    pub environment: String,
    pub claims: StandardClaims,
}
