use model::Tag;
use crate::git::client::error::GitClientError;

pub mod client;
pub mod model;

pub trait GitClient {
    fn create_release(&self, name: String, tag: Tag, body: String) -> Result<(), GitClientError>;
}
