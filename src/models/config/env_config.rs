use crate::models::config::jwt_config::JwtConfig;
use crate::models::config::{db_config::DbConfig, server_config::ServerConfig};
use confik::Configuration;

#[derive(Debug, Default, Configuration)]
pub struct EnvConfig {
    #[confik(from = DbConfig)]
    pub database: deadpool_postgres::Config,
    pub server: ServerConfig,
    pub jwt: JwtConfig,
}
