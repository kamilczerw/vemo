use config::ConfigError;
use crate::commands::error::CommandError;

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
