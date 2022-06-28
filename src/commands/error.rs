use std::fmt::{Display, Formatter};
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use crate::git::client::error::GitClientError;

#[derive(Debug)]
pub enum CommandError {
    /// Returned when parsing of shell command output cannot be converted to string
    ParseError(String),

    /// Returned when shell command failed
    ShellError(String),

    /// Returned when git client failed
    GitClientError(GitClientError)
}

impl From<Utf8Error> for CommandError {
    fn from(err: Utf8Error) -> Self {
        CommandError::ParseError(format!("{}", err))
    }
}

// impl From<FromUtf8Error> for CommandError {
//     fn from(err: FromUtf8Error) -> Self {
//         CommandError::ParseError(format!("{}", err))
//     }
// }

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::ParseError(err) => write!(f, "Parse error: {}", err),
            CommandError::ShellError(err) => write!(f, "Shell error: {}", err),
            CommandError::GitClientError(err) => write!(f, "Git client error: {}", err)
        }
    }
}
