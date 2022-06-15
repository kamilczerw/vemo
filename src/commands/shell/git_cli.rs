use std::process::Command;
use crate::commands::error::CommandError;
use crate::commands::shell::GitCli;

pub struct ShellGit {}

impl GitCli for ShellGit {

    /// List git tags ordered by version descending
    fn get_tags(&self, filter: String) -> Result<String, CommandError> {
        Self::fetch()?;
        Self::run(vec!["tag", "-l", filter.as_str(), "--sort=-v:refname"])
    }
}

impl ShellGit {
    fn run(args: Vec<&str>) -> Result<String, CommandError> {
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

    fn fetch() -> Result<(), CommandError> {
        Self::run(vec!["fetch", "--all", "--tags"]).map(|_| ())
    }
}
