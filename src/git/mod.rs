use config::Config;
use crate::commands::shell::git::{Repo, Tag};
use crate::git::client::error::GitClientError;
use crate::git::provider::error::GitProviderError;

pub mod client;
pub mod provider;

pub use provider::GitProvider;

pub trait GitClient {
    fn create_release(&self, name: String, tag: Tag, body: String) -> Result<(), GitClientError>;
}
