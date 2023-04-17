use std::{thread, time::Duration};

use sqlx::{
    postgres::{self, PgArguments, PgRow, PgSslMode},
    query::Query,
    Connection, PgPool, Postgres,
};

// TODO - make a wrapper around functions for transactions.
// TODO - tidy up, quite a lot of code re-use here.

// Configuration struct to set up database service.
pub struct Config {
    pub db_host: String,
    pub db_port: u16,
    pub db_username: String,
    pub db_password: String,
    pub db_schema: String,
    pub max_connections: u32,
    pub enable_ssl: PgSslMode,
}

// fn open_postgres_database() opens a new postgres connection pool.
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

// fn mutate_statement() creates a transaction to executate a insert or upate statement into the database.
pub async fn mutate_statement<'a>(
    db: &PgPool,
    query: Query<'a, Postgres, PgArguments>,
) -> Result<u64, sqlx::Error> {
    // Define a new transaction for this statement, catch any errors.
    let transaction = match db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => return Err(err),
    };

    // Executate the statement, if there is an issue with the update or insert,
    // we perform a rollback before return the error back up the stack.
    let result = match query.execute(db).await {
        Ok(result) => result,
        Err(err) => {
            if let Err(err) = transaction.rollback().await {
                return Err(err);
            }
            return Err(err);
        }
    };

    // If there are no errors at this point, we can then commit the transaction.
    if let Err(err) = transaction.commit().await {
        return Err(err);
    }

    Ok(result.rows_affected())
}

// fn query_single_row() queries one row from the database.
pub async fn query_single_row<'a>(
    db: &PgPool,
    query: Query<'a, Postgres, PgArguments>,
) -> Result<PgRow, sqlx::Error> {
    // Define a new transaction for this statement, catch any errors.
    let transaction = match db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => return Err(err),
    };

    // Executate the statement, if there is an issue with the update or insert,
    // we perform a rollback before return the error back up the stack.
    let result = match query.fetch_one(db).await {
        Ok(result) => result,
        Err(err) => {
            if let Err(err) = transaction.rollback().await {
                return Err(err);
            }
            return Err(err);
        }
    };

    // If there are no errors at this point, we can then commit the transaction.
    if let Err(err) = transaction.commit().await {
        return Err(err);
    }

    Ok(result)
}

// fn query_single_row() queries many rows from the database.
pub async fn query_many_rows<'a>(
    db: &PgPool,
    query: Query<'a, Postgres, PgArguments>,
) -> Result<Vec<PgRow>, sqlx::Error> {
    // Define a new transaction for this statement, catch any errors.
    let transaction = match db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => return Err(err),
    };

    // Executate the statement, if there is an issue with the update or insert,
    // we perform a rollback before return the error back up the stack.
    let result = match query.fetch_all(db).await {
        Ok(result) => result,
        Err(err) => {
            if let Err(err) = transaction.rollback().await {
                return Err(err);
            }
            return Err(err);
        }
    };

    // If there are no errors at this point, we can then commit the transaction.
    if let Err(err) = transaction.commit().await {
        return Err(err);
    }

    Ok(result)
}

// fn readiness_check() performs a ping to the database, every 5 seconds to see if the connection pool is still alive.
pub async fn readiness_check(
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
