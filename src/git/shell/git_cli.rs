use std::fmt::{Display, Formatter};
use std::process::Command;
use std::string::FromUtf8Error;
use log::{debug, warn};
use crate::commands::error::CommandError;
use crate::git::model::commit::Commit;
use crate::git::shell::GitCli;

pub struct ShellGit {}

impl GitCli for ShellGit {

    /// List git tags ordered by version descending
    fn get_tags(&self, filter: String) -> Result<String, CommandError> {
        Self::get_tags(filter).map_err(|e| CommandError::ShellError(e.to_string()))
    }

    fn get_config(&self, key: &str) -> Result<String, CommandError> {
        Self::get_config(key).map_err(|e| CommandError::ShellError(e.to_string()))
    }

    fn get_commits(&self, tag: Option<String>, dir: &str) -> Result<Vec<Commit>, CommandError> {
        Self::get_commits(tag, dir).map_err(|e| CommandError::ShellError(e.to_string()))
    }
}

impl ShellGit {
    fn run(args: Vec<&str>) -> Result<String, GitShellError> {
        let mut command = Command::new("git");
        command.args(args);
        debug!("Running git command: {:?}", &command);

        let output = command
            .output()
            .expect("Failed to execute git command");

        if !output.status.success() {
            let shell_error = String::from_utf8(output.stderr)?;
            Err(GitShellError::ShellError(shell_error))
        } else {
            Ok(String::from_utf8(output.stdout)?)
        }
    }

    fn fetch() -> Result<(), GitShellError> {
        Self::run(vec!["fetch", "--all", "--tags"]).map(|_| ())
    }

    /// List git tags ordered by version descending
    fn get_tags(filter: String) -> Result<String, GitShellError> {
        Self::fetch()?;
        Self::run(vec!["tag", "-l", filter.as_str(), "--sort=-v:refname"])
    }

    pub fn get_config(key: &str) -> Result<String, GitShellError> {
        Self::run(vec!["config", "--get", key])
    }

    fn get_commits(tag: Option<String>, dir: &str) -> Result<Vec<Commit>, GitShellError> {
        // %aN - Author name
        // %aE - Author email
        // %s - Subject
        // %H - Hash
        // %cI - Commit date ISO8601
        let format = "--pretty=format:%H;%aN;%aE;%s;%cI";
        let tags = match tag {
            None => String::from(""),
            Some(t) => format!("{}..origin/main", t)
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

pub enum GitShellError {
    ShellError(String),
    ParseError(String)
}


impl Display for GitShellError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            GitShellError::ShellError(e) => write!(f, "Shell error: {}", e),
            GitShellError::ParseError(e) => write!(f, "Parse error: {}", e)
        }
    }
}

impl From<FromUtf8Error> for GitShellError {
    fn from(err: FromUtf8Error) -> Self {
        GitShellError::ParseError(format!("{}", err))
    }
}
