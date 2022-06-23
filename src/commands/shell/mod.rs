mod git_cli;
pub(crate) mod git;

#[cfg(test)]
mod git_test;

use mockall::*;
use mockall::predicate::*;
use crate::commands::error::CommandError;

#[automock]
pub trait GitCli {
    /// Get filtered git tags
    fn get_tags(&self, filter: String) -> Result<String, CommandError>;

    /// Get git configuration for a given key
    fn get_config(&self, key: &str) -> Result<String, CommandError>;
}
