use confik::Configuration;
use crate::models::config::{db_config::DbConfig, server_config::ServerConfig};

#[derive(Debug, Default, Configuration)]
pub struct EnvConfig {
    #[confik(from = DbConfig)]
    pub database: deadpool_postgres::Config,
    pub server : ServerConfig
}
