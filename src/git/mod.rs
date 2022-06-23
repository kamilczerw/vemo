use crate::git::client::error::GitClientError;

pub mod client;

pub trait GitClient {
    fn create_release(&self) -> Result<(), GitClientError>;
}
