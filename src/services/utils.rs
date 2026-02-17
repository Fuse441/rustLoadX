
use config::{Config, ConfigError, File};
use crate::config::AppConfig;

pub fn get_configuration() -> Result<AppConfig, ConfigError> {
    println!("🔍 Loading configuration file...");

    let settings = match Config::builder()
        .add_source(File::with_name("config"))
        .build()
    {
        Ok(cfg) => {
            println!("✅ Config file loaded successfully");
            cfg
        }
        Err(e) => {
            eprintln!("❌ Failed to load config file: {e}");
            return Err(e);
        }
        
    };

    match settings.try_deserialize() {
        Ok(app_config) => {
            println!("✅ Config deserialized successfully");
            Ok(app_config)
        }
        Err(e) => {
            eprintln!("❌ Failed to deserialize config: {e}");
            Err(e)
        }
    }
}
