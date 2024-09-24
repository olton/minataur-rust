use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientConfig {
    pub server_address: String,
    pub server_port: u16,
    pub datetime_format: String,
    pub date_format: String,
    pub time_format: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MinaConfig {
    pub graphql_url: String,
    #[allow(dead_code)]
    pub client_status: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AppConfig {
    #[allow(dead_code)]
    pub server: ServerConfig,
    #[allow(dead_code)]
    pub client: ClientConfig,
    #[allow(dead_code)]
    pub database: DatabaseConfig,
    #[allow(dead_code)]
    pub mina: MinaConfig,
}

impl From<config::Config> for AppConfig {
    fn from(cfg: config::Config) -> Self {
        Self {
            server: cfg.get::<ServerConfig>("server").unwrap(),
            client: cfg.get::<ClientConfig>("client").unwrap(),
            database: cfg.get::<DatabaseConfig>("database").unwrap(),
            mina: cfg.get::<MinaConfig>("mina").unwrap(),
        }
    }
}

impl AppConfig {
    pub fn from_file(file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let cfg = config::Config::builder()
            .add_source(config::File::with_name(file))
            .build()?;

        cfg
            .try_into()
            .map_err(Into::into)
    }
}