use config::{Config, ConfigError};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: String,
    pub user: String,
    pub password: String,
    pub database_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // initialize config
        let config = Config::builder()
            .add_source(config::File::with_name("Config.toml"))
            .build()?;
        config.try_deserialize()
    }
}
