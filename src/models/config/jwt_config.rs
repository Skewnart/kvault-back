use confik::Configuration;

#[derive(Debug, Default, Configuration)]
pub struct JwtConfig {
    pub pk: String,
    pub sk: String,
    pub ttl: i64
}
