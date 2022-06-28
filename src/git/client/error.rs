use std::fmt::{Display, Formatter};
use crate::commands::error::CommandError;
use crate::commands::shell::git::GitCliError;
use crate::git::model::GitProvider;

#[derive(Debug)]
pub enum GitClientError {
    /// Git provider API token is missing
    MissingToken(GitProvider),

    /// Git provider API request error
    RequestError(reqwest::Error),

    /// Git command error
    GitCliError(GitCliError),

    /// Missing app config
    MissingAppConfig(String),
}

impl Display for GitClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GitClientError::MissingToken(provider) => {
                write!(
                    f,
                    "Missing token for a git provider, please set environment variable \"{}\", or \
                    add setting \"{}\" to ~/.vemo/config.toml",
                    provider.env_name(),
                    provider.setting_name()
                )
            },
            GitClientError::RequestError(err) => write!(f, "Request error: {}", err),
            GitClientError::GitCliError(error) => write!(f, "Git command error: {}", error),
            GitClientError::MissingAppConfig(app_name) => write!(f, "Missing app config for {}", app_name),
        }
    }
}

impl From<GitCliError> for GitClientError {
    fn from(err: GitCliError) -> Self {
        GitClientError::GitCliError(err)
    }
}
