use std::fmt::{Display, Formatter};
use crate::git;

#[derive(Debug)]
pub enum GitProviderError {
    ProviderNotSupported(git::Provider)
}

impl Display for GitProviderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GitProviderError::ProviderNotSupported(provider) => write!(
                f,
                "Git provider not supported: {}",
                provider
            )
        }
    }
}
