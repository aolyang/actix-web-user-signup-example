use config::{Config, ConfigError, Environment};
use serde::Deserialize;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Deserialize, Debug)]
pub struct Server {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub postgres_user: String,
    pub postgres_password: String,
    pub server: Server,
}

impl AppConfig {
    #[instrument]
    pub fn from_env() -> Result<Self, ConfigError> {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
        info!("loading configuration");

        let cfg = Config::builder()
            .add_source(Environment::default())
            .build()
            .expect("build config error");

        cfg.try_deserialize()
    }
}
