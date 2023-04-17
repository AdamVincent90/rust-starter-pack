use rust_starter_pack::{
    core::user::stores::user_db::user_db::UserStore,
    domain::system::auth::auth,
    lib::{database::database, logger::logger::Logger},
};
use std::error::Error;

// Lots of cleanup to do here.

pub async fn make_token(log: &Logger) -> Result<(), Box<dyn Error>> {
    // -----------------------------------------------------------
    // Custom postgres configuration, and initialsation.
    let database_config = database::Config {
        db_host: String::from("localhost"),
        db_port: 5439,
        db_username: String::from("postgres"),
        db_password: String::from("example"),
        db_schema: String::from("postgres"),
        max_connections: 2,
        enable_ssl: sqlx::postgres::PgSslMode::Disable,
    };

    let db = match database::open_postgres_database(database_config).await {
        Ok(db) => db,
        Err(err) => {
            return Err(err)?;
        }
    };

    // Get this from env.
    let auth = auth::new(auth::AuthConfig {
        enabled: true,
        key_id: String::from("72e8cca8-28a8-40e5-81bd-c1dbc7cfc5ee"),
        signing_method: jsonwebtoken::Algorithm::RS256,
        user_store: UserStore {
            logger: log.clone(),
            db: db,
        },
    });

    let token = match auth.new_token(1).await {
        Ok(token) => token,
        Err(err) => {
            log.error_w(
                format!("error making local token : {}", err.as_str()).as_str(),
                Some("SSL Make Token"),
            );
            std::process::exit(1);
        }
    };

    log.warn_w(
        format!("please copy the following token to use locally [{}]", token).as_str(),
        Some("SSL Make Token"),
    );

    Ok(())
}
