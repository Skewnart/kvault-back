use confik::Configuration;
use serde::Deserialize;

#[derive(Debug, Default, Configuration)]
pub struct EnvConfig {
    #[confik(from = DbConfig)]
    pub database: deadpool_postgres::Config,
    pub server : ServerConfig
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct DbConfig(deadpool_postgres::Config);

impl From<DbConfig> for deadpool_postgres::Config {
    fn from(value: DbConfig) -> Self {
        value.0
    }
}

impl confik::Configuration for DbConfig {
    type Builder = Option<Self>;
}

#[derive(Debug, Default, Configuration)]
pub struct ServerConfig {
    pub port : u16
}