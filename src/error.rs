use config::ConfigError;
use crate::commands::error::CommandError;
use crate::git::client::error::GitClientError;

pub struct AppError {
    pub message: String,
    pub code: i32
}

impl From<ConfigError> for AppError {
    fn from(err: ConfigError) -> Self {
        let message = match &err {
            ConfigError::FileParse { .. } => {
                format!("Failed to parse config, {}", err)
            }
            _ => format!("{}", err)
        };

        AppError { message, code: 1 }
    }
}

impl From<CommandError> for AppError {
    fn from(err: CommandError) -> Self {
        match err {
            CommandError::ParseError(message) => { AppError { message, code: 1 } }
            CommandError::ShellError(message) => { AppError { message, code: 1 } }
        }
        // TODO: implement better mapping for command errors
        // AppError { message: format!(""), code: 1 }
    }
}

impl From<GitClientError> for AppError {
    fn from(err: GitClientError) -> Self {
        match err {
            GitClientError::UnexpectedError(message) => { AppError { message, code: 1 } }
            GitClientError::MissingToken(provider) => { AppError {
                message: format!("Missing token for a git provider, please set environment variable \"{}\", or add setting \"{}\" to ~/.vemo/config.toml", provider.env_name(), provider.setting_name()).to_string(),
                code: 1
            } }
            GitClientError::UnsupportedProvider(provider) => { AppError { message: format!("Unsupported provider: {}", provider), code: 1 } }
            GitClientError::RequestError(err) => { AppError { message: format!("{}", err), code: 1 } }
        }
    }
}
