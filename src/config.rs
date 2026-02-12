use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub app_name: String,
    pub max_connections: u32,
    pub debug: bool,
}


impl Config {

    pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}

   pub fn validate(&self) -> Result<(), String> {
        if self.max_connections == 0 {
            return Err("max_connections must be greater than 0".into());
        }

        if self.app_name.trim().is_empty() {
            return Err("app_name cannot be empty".into());
        }

        Ok(())
    }
}

