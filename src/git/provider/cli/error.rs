use std::fmt::Display;
use crate::git::shell::git_cli::GitShellError;

#[derive(Debug)]
pub enum CliProviderError {
    UnexpectedError(String)
}

impl Display for CliProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliProviderError::UnexpectedError(msg) => write!(f, "Unexpected error: {}", msg)
        }
    }
}

impl From<GitShellError> for CliProviderError {
    fn from(err: GitShellError) -> Self {
        CliProviderError::UnexpectedError(format!("{}", err))
    }
}
