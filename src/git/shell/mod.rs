use crate::commands::error::CommandError;
use crate::git::model::commit::Commit;

pub(crate) mod git_cli;
#[cfg(test)]
mod git_test;

pub trait GitCli {
    /// Get filtered git tags
    fn get_tags(&self, filter: String) -> Result<String, CommandError>;

    /// Get git configuration for a given key
    fn get_config(&self, key: &str) -> Result<String, CommandError>;

    /// Get git commits for a given tag and directory
    fn get_commits(&self, tag: Option<String>, dir: &str) -> Result<Vec<Commit>, CommandError>;
}

#[cfg(test)]
mod mock {
    use mockall::mock;
    use crate::commands::error::CommandError;
    use crate::git::model::commit::Commit;

    mock! {
        pub GitCli {}
        impl super::GitCli for GitCli {
            fn get_tags(&self, filter: String) -> Result<String, CommandError>;
            fn get_config(&self, key: &str) -> Result<String, CommandError>;
            fn get_commits(&self, tag: Option<String>, dir: &str) -> Result<Vec<Commit>, CommandError>;
        }
    }
}
