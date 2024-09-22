use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
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
    pub database: DatabaseConfig,
    #[allow(dead_code)]
    pub mina: MinaConfig,
}

impl From<config::Config> for AppConfig {
    fn from(cfg: config::Config) -> Self {
        Self {
            server: cfg.get::<ServerConfig>("server").unwrap(),
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