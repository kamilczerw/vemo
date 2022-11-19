use std::fmt::{Display, Formatter};
use crate::git;

#[derive(Debug)]
pub enum GitProviderError {
    /// Failed to initialize git provider
    InitError(String),

    ProviderNotSupported(git::Provider)
}

impl Display for GitProviderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GitProviderError::InitError(message) => {
                write!(
                    f,
                    "Failed to initialize git provider: {}",
                    message
                )
            }
            GitProviderError::ProviderNotSupported(provider) => write!(
                f,
                "Git provider not supported: {}",
                provider
            )
        }
    }
}
