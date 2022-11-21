use std::fmt::{Display, Formatter};
use downcast_rs::{DowncastSync, impl_downcast};
use crate::cfg::Config;
use crate::git;
use crate::git::{Git, git_provider};
use crate::git::model::repo::Repo;
use crate::git::model::tag::Tag;
use crate::git::provider::error::GitProviderError;

pub mod error;

mod github;
mod provider_test;
pub(crate) mod cli;

pub use cli::Provider as CliProvider;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Default)]
pub enum Provider {
    #[default]
    Github,
    Unknown
}


pub trait GitProvider: DowncastSync {
    fn find_latest_tag(&self, app_name: &str) -> Result<Option<Tag>, GitProviderError>;

    fn release(&self, name: &str, tag: Tag, body: String) -> Result<(), GitProviderError>;
}

// Used for testing to test if the constructed provider is of the correct type
impl_downcast!(sync GitProvider);


pub fn new(cli: &CliProvider, config: &Config) -> Result<Box<dyn GitProvider>, GitProviderError> {
    match cli.repo.provider {
        Provider::Github => Ok(Box::new(github::Provider::new())),
        _ => Err(GitProviderError::ProviderNotSupported(cli.repo.provider.clone()))
    }
}

impl Display for Provider {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Provider::Github => write!(f, "github"),
            Provider::Unknown => write!(f, "unknown")
        }
    }
}
