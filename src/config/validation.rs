use super::{Config};
use super::error::ConfigError;

pub fn validate(config: &Config) -> Result<(), ConfigError> {
    if config.app.name.trim().is_empty() {
        return Err(ConfigError::Validation(
            "App name cannot be empty".into(),
        ));
    }

    if config.database.pool_size == 0 {
        return Err(ConfigError::Validation(
            "Database pool_size must be greater than 0".into(),
        ));
    }

    Ok(())
}
