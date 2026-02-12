mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::load_config()?;

    config.validate()
        .map_err(|e| format!("Config validation failed: {}", e))?;

    println!("Starting {}...", config.app_name);

    if config.debug {
        println!("Debug mode enabled");
    }

    println!("Max connections: {}", config.max_connections);

    Ok(())
}
