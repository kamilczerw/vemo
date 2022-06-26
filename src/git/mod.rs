use model::Tag;
use crate::git::client::error::GitClientError;
use crate::git::model::Commit;

pub mod client;
pub mod model;

pub trait GitClient {
    fn create_release(&self, name: String, tag: Tag, body: String) -> Result<(), GitClientError>;

    fn latest_release(&self, name: &str) -> Result<Option<Tag>, GitClientError>;

    fn get_changelog(&self, tag: Option<Tag>, app_name: &str) -> Result<Vec<Commit>, GitClientError>;
}
