use serde::Deserialize;
use ultimate_rust_service::foundation::logger::logger;

// Every main.rs executable, in most cases, should have a config for the app, these configs
// aim to provide the app context to where or what they are performing business logic to, or for.

// The main config struct, this is mapped from your env vars.
pub struct AppConfig {
    pub app: AppSettings,
    // Your custom configs go below as a struct.
    pub web: WebSettings,
    pub db: DatabaseSettings,
}

#[derive(Deserialize)]
pub struct AppSettings {
    pub version: String,
    pub environment: String,
}

// We derive Deserialize to allow mappings to work on nested structs.
#[derive(Deserialize)]
pub struct WebSettings {
    pub address: String,
    pub port: u16,
}

// We derive Deserialize to allow mappings to work on nested structs.
#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub schema: String,
}

// May aswell add this as some form of foundation package, i want this to handle defaults in a cleaner way for devs to use
// Right now, to load in a new setting you will need to add a function like this, the struct, add to the config and then
// provide defaults in main.rs.
impl AppConfig {
    pub fn load_from_env(self, logger: &logger::Logger) -> Result<AppConfig, envy::Error> {
        dotenvy::dotenv().ok();

        let app = envy::from_env::<AppSettings>().unwrap_or_else(|err| {
            logger.warn_w(
                "Could not load app settings from env, reverting to default : reason :",
                Some(err),
            );
            return self.app;
        });

        let web = envy::prefixed("WEB_")
            .from_env::<WebSettings>()
            .unwrap_or_else(|err| {
                logger.warn_w(
                    "Could not load web settings from env, reverting to default : reason :",
                    Some(err),
                );
                return self.web;
            });

        let db = envy::prefixed("DB_")
            .from_env::<DatabaseSettings>()
            .unwrap_or_else(|err| {
                logger.warn_w(
                    "Could not load db settings from env, reverting to default : reason :",
                    Some(err),
                );
                return self.db;
            });

        Ok(AppConfig { app, web, db })
    }
}
