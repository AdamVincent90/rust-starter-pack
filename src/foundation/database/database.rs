use std::time::Duration;

use sqlx::postgres;

pub struct Config {
    pub db_host: String,
    pub db_username: String,
    pub db_password: String,
    pub db_schema: String,
    pub max_connections: u32,
}

pub async fn open_postgres_database(config: Config) -> Result<postgres::PgPool, sqlx::Error> {
    let connection_url = format!(
        "postgres://{}:{}@{}/{}",
        config.db_username, config.db_password, config.db_host, config.db_schema
    );

    let postgres_db = match postgres::PgPoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&connection_url)
        .await
    {
        Ok(db) => db,
        Err(err) => return Err(err),
    };

    Ok(postgres_db)
}
