use crate::commands::error::CommandError;
use crate::git::model::author::Author;
use chrono::ParseError;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Commit {
    pub hash: String,
    pub message: String,
    pub author: Author,
    pub date: chrono::DateTime<chrono::Utc>
}

impl Commit {
    /// Create a new commit from a git log line
    /// format:%H;%aN;%aE;%s;%cI
    pub(crate) fn from_line(line_output: &str) -> Result<Self, CommandError> {
        let mut line = line_output.to_string();
        line.retain(|c| c != '\n');
        let mut parts = line.split(';');
        let hash = parts.next().ok_or(CommandError::ShellError("Failed to parse commit hash".to_string()))?.to_string();
        let author_name = parts.next().ok_or(CommandError::ShellError("Failed to parse commit author's name".to_string()))?.to_string();
        let author_email = parts.next().ok_or(CommandError::ShellError("Failed to parse commit author's email".to_string()))?.to_string();
        let message = parts.next().ok_or(CommandError::ShellError("Failed to parse commit message".to_string()))?.to_string();
        let date_string = parts.next().ok_or(CommandError::ShellError("Failed to parse commit date".to_string()))?;
        let date = chrono::DateTime::parse_from_str(date_string, "%Y-%m-%dT%H:%M:%S%z")?.with_timezone(&chrono::Utc);
        let author = Author {
            name: author_name,
            email: author_email
        };
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
        CommandError::ParseError(format!("Failed to parse date, reason: {:?}", e))
    }
}
