use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Server {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize, Debug)]
pub struct DbConfig {
    pub user: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub server: Server,
    pub db: DbConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let cfg = Config::builder()
            .add_source(Environment::default())
            .build()
            .expect("build config error");

        cfg.try_deserialize()
    }
}
