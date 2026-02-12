mod config;

use config::loader::load_from_file;
use config::validation::validate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_from_file("config.toml")?;
    if config.debug {
        println!("Debug mode enabled");
    }

    println!("{}", config.debug);

    validate(&config)?;

    println!("Starting {}...", config.app.name);
    println!("Listening on port {}", config.app.port);
    println!("Database URL: {}", config.database.url);

    Ok(())
}
