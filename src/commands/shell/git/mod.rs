mod git;

use std::fmt::Display;
use std::string::FromUtf8Error;
pub use git::Git;

#[derive(Debug)]
pub enum GitCliError {
    ParseError(String),
    ShellError(String),
}

impl Display for GitCliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GitCliError::ParseError(err) => write!(f, "Parse error: {}", err),
            GitCliError::ShellError(err) => write!(f, "Shell error: {}", err),
        }
    }
}

impl From<FromUtf8Error> for GitCliError {
    fn from(err: FromUtf8Error) -> Self {
        GitCliError::ParseError(format!("{}", err))
    }
}
