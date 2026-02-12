pub mod error;
pub mod loader;
pub mod validation;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    pub debug: bool,
}



#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppConfig {
    pub name: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
}

fn default_port() -> u16 {
    8080
}
