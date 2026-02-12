use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("Failed to parse config: {0}")]
    ParseToml(#[from] toml::de::Error),

    #[error("Validation error: {0}")]
    Validation(String),
}
