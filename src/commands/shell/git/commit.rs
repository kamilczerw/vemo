use chrono::ParseError;
use crate::commands::error::CommandError;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Commit {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub date: chrono::DateTime<chrono::Utc>
}

impl Commit {
    /// Create a new commit from a git log line
    /// format:%aN;%aE;%H;%s;%cI
    pub(crate) fn from_line(line_output: &str) -> Result<Self, CommandError> {
        let mut line = line_output.to_string();
        line.retain(|c| c != '\n');
        let mut parts = line.split(';');
        let hash = parts.next().ok_or(CommandError::ShellError("Failed to parse commit hash".to_string()))?.to_string();
        let message = parts.next().ok_or(CommandError::ShellError("Failed to parse commit message".to_string()))?.to_string();
        let author = parts.next().ok_or(CommandError::ShellError("Failed to parse commit author".to_string()))?.to_string();
        let date_string = parts.next().ok_or(CommandError::ShellError("Failed to parse commit date".to_string()))?;
        let date = chrono::DateTime::parse_from_str(date_string, "%Y-%m-%d %H:%M:%S %z")?.with_timezone(&chrono::Utc);

        Ok(Self {
            hash,
            message,
            author,
            date
        })
    }
}

impl From<ParseError> for CommandError {
    fn from(e: ParseError) -> Self {
        CommandError::ParseError(e.to_string())
    }
}
