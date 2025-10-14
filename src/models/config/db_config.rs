use serde::Deserialize;

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
