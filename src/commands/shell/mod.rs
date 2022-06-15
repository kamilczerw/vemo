use std::process::Command;
use crate::commands::error::CommandError;

pub struct Git {}

impl Git {
    pub fn run(args: Vec<&str>) -> Result<String, CommandError> {
        let output = Command::new("git").args(args)
            .output()
            .expect("Failed to execute git command");

        if !output.status.success() {
            let shell_error = String::from_utf8(output.stderr)?;
            Err(CommandError::ShellError(shell_error))
        } else {
            Ok(String::from_utf8(output.stdout)?)
        }
    }

    pub fn fetch() -> Result<(), CommandError> {
        // TODO: cache the result so the command runs faster most of the time and it only fetches
        //       the tags from after certain amount of time.
        Self::run(vec!["fetch", "--all", "--tags"]).map(|_| ())
    }

    /// List git tags ordered by version descending
    pub fn get_tags(filter: String) -> Result<String, CommandError> {
        Self::fetch()?;
        Self::run(vec!["tag", "-l", filter.as_str(), "--sort=-v:refname"])
    }

    // pub fn get_latest_tag(format: String) -> Result<Option<String>, CommandError> {
    //     Self::get_tags(format).map(|tag| )
    // }
}
