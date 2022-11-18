use std::str::Utf8Error;
use std::string::FromUtf8Error;
use crate::git::client::error::GitClientError;
use crate::git::provider::error::GitProviderError;

#[derive(Debug)]
pub enum CommandError {
    /// Returned when parsing of shell command output cannot be converted to string
    ParseError(String),

    /// Returned when shell command failed
    ShellError(String),

    /// Returned when git client failed
    GitClientError(GitClientError),

    /// Returned when unknown error occurred
    UnexpectedError(String)
}

impl From<Utf8Error> for CommandError {
    fn from(err: Utf8Error) -> Self {
        CommandError::ParseError(format!("{}", err))
    }
}

impl From<FromUtf8Error> for CommandError {
    fn from(err: FromUtf8Error) -> Self {
        CommandError::ParseError(format!("{}", err))
    }
}

impl From<GitProviderError> for CommandError {
    fn from(err: GitProviderError) -> Self {
        CommandError::ShellError(format!("{}", err))
    }
}
