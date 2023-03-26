// This will be the core business logic to handle a user.

use sqlx::PgPool;

use crate::foundation::logger::logger::Logger;

#[derive(Clone)]
pub struct Core {
    pub logger: Logger,
    pub db: PgPool,
}

pub fn new_core(logger: &Logger, db: &PgPool) -> Core {
    Core {
        logger: logger.clone(),
        db: db.clone(),
    }
}

impl Core {}
