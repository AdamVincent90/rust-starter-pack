use std::{thread, time::Duration};

use sqlx::{
    postgres::{self, PgSslMode},
    Connection, Executor, PgPool, Statement,
};

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
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(connection_options)
        .await
    {
        Ok(db) => db,
        Err(err) => return Err(err),
    };

    Ok(postgres_db)
}

pub async fn ping_postgres_server(
    db: &sqlx::postgres::PgPool,
    max_attempts: u8,
) -> Result<(), sqlx::Error> {
    for i in 1..=max_attempts {
        thread::sleep(Duration::from_secs(5));

        let connection = &mut db.acquire().await?;

        match connection.ping().await {
            Ok(_) => {
                break;
            }
            Err(e) => {
                if i == max_attempts {
                    return Err(e);
                }
            }
        }
    }

    if let Err(err) = sqlx::query("SELECT true").fetch_one(db).await {
        return Err(err);
    }

    Ok(())
}

pub async fn execute_statement(db: &PgPool, query: &str) -> Result<(), sqlx::Error> {
    // Prepare Query
    let statement = match db.prepare(&query).await {
        Ok(statement) => statement,
        Err(err) => return Err(err),
    };

    // Log Query
    // TODO

    // Sanitise Query (if required)
    // TODO

    // Transaction Begin
    // TODO

    // Execute Query
    match db.execute(statement.sql()).await {
        Ok(result) => result,
        Err(err) => return Err(err), // Rollback
    };

    // Commit
    // TODO

    Ok(())
}

pub async fn query_single_row(db: &PgPool, query: &str) -> Result<(), sqlx::Error> {
    // Prepare Query
    let statement = match db.prepare(&query).await {
        Ok(statement) => statement,
        Err(err) => return Err(err),
    };

    // Log Query
    // TODO

    // Sanitise Query (if required)
    // TODO

    // Transaction Begin
    // TODO

    // Execute Query
    match db.execute(statement.sql()).await {
        Ok(result) => result,
        Err(err) => return Err(err), // Rollback
    };

    // Commit
    // TODO

    Ok(())
}

pub async fn query_many_rows(db: &PgPool, query: &str) -> Result<(), sqlx::Error> {
    // Prepare Query
    let statement = match db.prepare(&query).await {
        Ok(statement) => statement,
        Err(err) => return Err(err),
    };

    // Log Query
    // TODO

    // Sanitise Query (if required)
    // TODO

    // Transaction Begin
    // TODO

    // Execute Query
    match db.execute(statement.sql()).await {
        Ok(result) => result,
        Err(err) => return Err(err), // Rollback
    };

    // Commit
    // TODO

    Ok(())
}
