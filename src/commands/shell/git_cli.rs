use std::process::Command;
use crate::commands::error::CommandError;
use crate::commands::shell::GitCli;
use crate::commands::shell::git::Commit;
use log::{debug, warn};

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

    fn get_commits(&self, tag: Option<String>, dir: &str) -> Result<Vec<Commit>, CommandError> {
        // %aN - Author name
        // %aE - Author email
        // %s - Subject
        // %H - Hash
        // %cI - Commit date ISO8601
        let format = "--pretty=format:%H;%aN;%aE;%s;%cI";
        let tags = match tag {
            None => String::from(""),
            Some(t) => format!("{}..HEAD", t)
        };
        let git_command = vec![
            "log",
            "--oneline",
            "--decorate",
            format,
            &tags,
            "--",
            dir
        ];

        Self::run(git_command).map(|output| {
            let mut commits = vec![];
            for line in output.lines() {
                Commit::from_line(line).map(|c| {
                    commits.push(c);
                }).map_err(|e| {
                    warn!("Failed to parse commit line. Skipping!");
                    debug!("Reason: {:?}", e);
                }).expect("TODO: panic message");
            }
            commits
        })
    }
}

impl ShellGit {
    fn run(args: Vec<&str>) -> Result<String, CommandError> {
        let mut command = Command::new("git");
        command.args(args);
        debug!("Running git command: {:?}", &command);

        let output = command
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
