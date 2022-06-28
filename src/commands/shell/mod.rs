pub mod git_cli;
pub mod git;

#[cfg(test)]
mod git_test;

use mockall::*;
use mockall::predicate::*;
use crate::commands::shell::git::GitCliError;
use crate::git::model::Change;

#[automock]
pub trait GitCli {
    /// Get filtered git tags
    fn get_tags(&self, filter: String) -> Result<String, GitCliError>;

    /// Get git configuration for a given key
    fn get_config(&self, key: &str) -> Result<String, GitCliError>;

    /// Get git commits for a given tag and directory
    fn get_commits(&self, tag: Option<String>, dir: &str) -> Result<Vec<Change>, GitCliError>;
}
