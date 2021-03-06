use std::fmt::{Display, Formatter};
use crate::commands::shell::git::GitProvider;

#[derive(Debug)]
pub enum GitClientError {
    /// Git provider API token is missing
    MissingToken(GitProvider),

    /// Git provider is not supported
    UnsupportedProvider(GitProvider),

    /// Git provider API request error
    RequestError(reqwest::Error),
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
            GitClientError::UnsupportedProvider(provider) => write!(f, "Unsupported provider: {}", provider),
            GitClientError::RequestError(err) => write!(f, "Request error: {}", err),
        }
    }
}
