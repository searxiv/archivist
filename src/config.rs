#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub addr: String,
    pub port: u16,
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database_url: "postgres://postgres:password@localhost/arxiv".to_string(),
            addr: "0.0.0.0".to_string(),
            port: 9000,
            log_level: "info".to_string(),
        }
    }
}

use figment::{
    value::{Dict, Map},
    Error, Metadata, Profile, Provider,
};

impl Provider for Config {
    fn metadata(&self) -> Metadata {
        Metadata::named("Archiver config")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        figment::providers::Serialized::defaults(Config::default()).data()
    }
}
