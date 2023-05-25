use config::{Config, ConfigError, File, FileFormat};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Logging {
    pub folder_path: String,
}

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
    pub logging: Logging,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // initialize config
        let config = Config::builder()
            // .add_source(File::new("/etc/spamvoid/config.toml", FileFormat::Toml))
            .add_source(File::new("./config.toml", FileFormat::Toml))
            .build()?;
        config.try_deserialize()
    }
}
