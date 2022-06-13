use std::collections::HashMap;
use std::env;
use std::path::Path;
use config::{ConfigError, Source};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub path: Option<Path>,
    pub ignore: bool
}

#[derive(Debug, Clone)]
pub struct Config {
    pub format: String,
    pub debug: bool,
    pub apps: HashMap<String, AppConfig>
}

impl Config {
    pub fn init() -> Result<Config, ConfigError> {

        let path = env::current_dir().map_err(|_| {
            ConfigError::Message(String::from("Failed to open current directory."))
        })?;

        let config_path = format!("{}/.mov.toml", path.display());
        let config_file = Path::new(&config_path);
        let settings = config::Config::builder();

        let settings = if config_file.exists() {
            settings.add_source(config::File::from(config_file))
        } else { settings };

        let settings = settings
            .add_source(config::Environment::with_prefix("MOV"))
            .build()?;

        let format = settings.get_string("format")
            .unwrap_or(String::from("{app_name}/v{version}"));

        let debug = settings.get_bool("debug").unwrap_or(false);

        Ok(Config { format, debug, apps: HashMap::new() })
    }
}
