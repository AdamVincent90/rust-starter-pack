use crate::{business::system::auth::auth::Auth, dependency::logger::logger::Logger};
use sqlx::PgPool;

// AuthContext contains all the state required to succefully auth a request.
#[derive(Clone)]
pub struct AuthContext {
    pub auth: Auth,
}

// AuditContext contains all the state required to succefully audit a request.
#[derive(Clone)]
pub struct AuditContext {
    pub db: PgPool,
}

// ErrorContext contains all the state required to succefully handle request errors.
#[derive(Clone)]
pub struct ErrorContext {
    pub log: Logger,
}

// LoggingContext contains all the state required to succefully log a request.
#[derive(Clone)]
pub struct LoggingContext {
    pub log: Logger,
}
