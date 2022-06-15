use std::str::Utf8Error;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum CommandError {
    /// Returned when parsing of shell command output cannot be converted to string
    ParseError(String),

    /// Returned when shell command failed
    ShellError(String)
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
