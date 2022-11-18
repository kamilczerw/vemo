use std::collections::HashMap;
use std::env;
use std::path::Path;
use config::{Config as Cfg, ConfigError, ValueKind};

#[cfg(test)]
mod config_test;

pub static DEFAULT_TAG_FORMAT: &str = "{app_name}/v{version}";

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub path: Option<String>
}

#[derive(Debug, Clone)]
pub struct Config {
    pub format: String,
    pub debug: bool,

    #[deprecated(since="1.0.0", note="please use `providers` instead")]
    pub gh_token: Option<String>,
    pub apps: HashMap<String, AppConfig>
}

impl Config {
    pub fn init() -> Result<Config, ConfigError> {
       let settings = Config::read_config()?;

        let format = settings.get_string("format")
            .unwrap_or(String::from(DEFAULT_TAG_FORMAT));

        let debug = settings.get_bool("debug").unwrap_or(false);
        let gh_token = match settings.get_string("github.token") {
            Ok(token) => Some(token),
            _ => None
        };

        let app_configs = Self::get_app_configs(settings)?;

        Ok(Config { format, debug, gh_token, apps: app_configs })
    }

    pub fn app_path(&self, app_name: &str) -> Option<String> {
        self.apps.get(app_name)
            .map(|app| app.path.clone())
            .unwrap_or(None)
    }

    fn read_config() -> Result<Cfg, ConfigError> {
        let current_dir = env::current_dir().map_err(|_| {
            ConfigError::Message(String::from("Failed to open current directory."))
        })?;

        let home_dir = dirs::home_dir()
            .ok_or(ConfigError::Message(String::from("Failed to get home directory.")))?;
        let home_config = home_dir.join(".config/vemo/config.toml");

        let config_path = format!("{}/.vemo.toml", current_dir.display());
        let config_file = Path::new(&config_path);

        let settings = config::Config::builder();

        let settings = if home_config.exists() {
            settings.add_source(config::File::from(home_config.as_path()))
        } else { settings };

        let settings = if config_file.exists() {
            settings.add_source(config::File::from(config_file))
        } else { settings };

        return settings
            .add_source(config::Environment::with_prefix("VEMO"))
            .build();
    }

    fn get_app_configs(settings: Cfg) -> Result<HashMap<String, AppConfig>, ConfigError> {
        let mut app_configs: HashMap<String, AppConfig> = HashMap::new();

        // TODO: handle error instead of using .unwrap()
        for (app_name, value) in settings.get_table("app").unwrap().iter() {
            match &value.kind {
                ValueKind::Table(t) => {
                    let path = t.get("path").map(|v| {
                        v.clone().into_string().map(Some)
                    }).unwrap_or(Ok(None));

                    let app_config = AppConfig { path: path? };
                    app_configs.insert(app_name.clone(), app_config);
                }
                _ => { }
            };
        }

        return Ok(app_configs)
    }
}
