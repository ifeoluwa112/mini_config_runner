use std::fs;

use super::Config;
use super::error::ConfigError;

pub fn load_from_file(path: &str) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
