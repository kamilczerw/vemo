use model::Release;
use crate::git::client::error::GitClientError;
use crate::git::model::Change;

pub mod client;
pub mod model;

pub trait GitClient {
    fn create_release(&self, name: String, tag: Release, body: String) -> Result<(), GitClientError>;

    fn latest_release(&self, name: &str) -> Result<Option<Release>, GitClientError>;

    fn get_changelog(&self, tag: Option<Release>, app_name: &str) -> Result<Vec<Change>, GitClientError>;

    fn list_latest_releases(&self) -> Result<Vec<Release>, GitClientError>;
}
