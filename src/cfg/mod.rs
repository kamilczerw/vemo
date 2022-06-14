use std::collections::HashMap;
use std::env;
use std::path::Path;
use config::{Config as Cfg, ConfigError, Source, ValueKind};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub path: Option<String>
}

#[derive(Debug, Clone)]
pub struct Config {
    pub format: String,
    pub debug: bool,
    pub apps: HashMap<String, AppConfig>
}

impl Config {
    pub fn init() -> Result<Config, ConfigError> {
       let settings = Config::read_config()?;

        let format = settings.get_string("format")
            .unwrap_or(String::from("{app_name}/v{version}"));

        let debug = settings.get_bool("debug").unwrap_or(false);

        let app_configs = Self::get_app_configs(settings)?;

        Ok(Config { format, debug, apps: app_configs })
    }

    fn read_config() -> Result<Cfg, ConfigError> {
        let current_dir = env::current_dir().map_err(|_| {
            ConfigError::Message(String::from("Failed to open current directory."))
        })?;

        let config_path = format!("{}/.vemo.toml", current_dir.display());
        let config_file = Path::new(&config_path);

        let settings = config::Config::builder();

        let settings = if config_file.exists() {
            settings.add_source(config::File::from(config_file))
        } else { settings };

        return settings
            .add_source(config::Environment::with_prefix("VEMO"))
            .build();
    }

    fn get_app_configs(settings: Cfg) -> Result<HashMap<String, AppConfig>, ConfigError> {
        let mut app_configs: HashMap<String, AppConfig> = HashMap::new();

        for (key, value) in settings.collect().unwrap() {
            match value.kind {
                ValueKind::Table(t) => {
                    let path = t.get("path").map(|v| {
                        v.clone().into_string().map(Some)
                    }).unwrap_or(Ok(None));

                    let app_config = AppConfig { path: path? };
                    app_configs.insert(key, app_config);
                }
                _ => { }
            };
        }

        return Ok(app_configs)
    }
}
