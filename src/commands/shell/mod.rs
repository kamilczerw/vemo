mod git_cli;
pub(crate) mod git;

#[cfg(test)]
mod git_test;

use crate::commands::error::CommandError;

pub trait GitCli {
    /// Get filtered git tags
    fn get_tags(&self, filter: String) -> Result<String, CommandError>;
}
