use crate::commands::shell::git::Tag;
use crate::git::client::error::GitClientError;

pub mod client;

pub trait GitClient {
    fn create_release(&self, name: String, tag: Tag, body: String) -> Result<(), GitClientError>;
}
