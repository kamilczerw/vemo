use std::process::Command;
use crate::commands::error::CommandError;
use crate::commands::shell::GitCli;
use crate::commands::shell::git::Commit;

pub struct ShellGit {}

impl GitCli for ShellGit {

    /// List git tags ordered by version descending
    fn get_tags(&self, filter: String) -> Result<String, CommandError> {
        Self::fetch()?;
        Self::run(vec!["tag", "-l", filter.as_str(), "--sort=-v:refname"])
    }

    fn get_config(&self, key: &str) -> Result<String, CommandError> {
        Self::run(vec!["config", "--get", key])
    }

    fn get_commits(&self, tag: &str, dir: &str) -> Result<Vec<Commit>, CommandError> {
        // Self::run(vec!["log", "--oneline", "--decorate", "--pretty=format:%H", &format!("HEAD..{}", tag), "--", dir])
        todo!()
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
