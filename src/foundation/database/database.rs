use std::{thread, time::Duration};

use sqlx::{
    postgres::{self, PgSslMode},
    Connection,
};

use crate::foundation::logger::logger;

pub struct Config {
    pub db_host: String,
    pub db_port: u16,
    pub db_username: String,
    pub db_password: String,
    pub db_schema: String,
    pub max_connections: u32,
    pub enable_ssl: PgSslMode,
}

pub async fn open_postgres_database(config: Config) -> Result<postgres::PgPool, sqlx::Error> {
    let connection_options = postgres::PgConnectOptions::new()
        .database(&config.db_schema)
        .ssl_mode(config.enable_ssl)
        .username(&config.db_username)
        .password(&config.db_password)
        .host(&config.db_host)
        .port(config.db_port);

    let postgres_db = match postgres::PgPoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(Duration::from_secs(10))
        .connect_with(connection_options)
        .await
    {
        Ok(db) => db,
        Err(err) => return Err(err),
    };

    Ok(postgres_db)
}

pub async fn ping_connection(
    db: &sqlx::postgres::PgPool,
    log: &logger::Logger,
    max_attempts: u8,
) -> Result<(), sqlx::Error> {
    for i in 1..=max_attempts {
        thread::sleep(Duration::from_secs(5));

        let connection = &mut db.acquire().await?;

        match connection.ping().await {
            Ok(_) => {
                log.info_w("connection ping found", Some(connection));
                break;
            }
            Err(e) => {
                log.error_w("connection ping failed", Some(e));
                if i == max_attempts {
                    return Err(sqlx::Error::PoolClosed);
                }
            }
        }
    }

    let row: (bool,) = sqlx::query_as("SELECT true").fetch_one(db).await?;

    log.info_w("ping confirmed : check completed", Some(row));

    Ok(())
}
