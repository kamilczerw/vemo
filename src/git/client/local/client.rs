
use crate::git::model::{Change, Release};
use crate::commands::shell::GitCli;
use crate::commands::shell::git_cli::ShellGit;
use crate::git::client::error::GitClientError;
use crate::git::GitClient;

pub struct LocalClient {
    git: Box<dyn GitCli>,
    tag_format: String // TODO: is this needed?
}

impl LocalClient {
    pub fn init(tag_format: String) -> LocalClient {
        Self::new(Box::new(ShellGit {}), tag_format)
    }

    pub fn new(git: Box<dyn GitCli>, tag_format: String) -> LocalClient {
        LocalClient { git, tag_format }
    }
}

impl GitClient for LocalClient {
    fn create_release(&self, name: String, tag: Release, body: String) -> Result<(), GitClientError> {
        todo!()
    }

    fn latest_release(&self, name: &str) -> Result<Option<Release>, GitClientError> {
        todo!()
    }

    fn get_changelog(&self, tag: Option<Release>, app_name: &str) -> Result<Vec<Change>, GitClientError> {
        todo!()
    }

    fn list_latest_releases(&self) -> Result<Vec<Release>, GitClientError> {
        todo!()
    }
}
