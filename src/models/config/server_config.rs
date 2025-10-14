use confik::Configuration;

#[derive(Debug, Default, Configuration)]
pub struct ServerConfig {
    pub port : u16
}