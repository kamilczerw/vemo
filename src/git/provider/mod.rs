use std::fmt::{Display, Formatter};
use crate::cfg::Config;
use crate::git;
use crate::git::{Git, git_provider};
use crate::git::model::repo::Repo;
use crate::git::model::tag::Tag;
use crate::git::provider::error::GitProviderError;

pub mod error;

mod github;
mod cli;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Provider {
    Github,
    Unknown
}

pub trait GitProvider {
    fn find_latest_tag(&self, app_name: &str) -> Result<Option<Tag>, GitProviderError>;

    fn release(&self, name: &str, tag: Tag, body: String) -> Result<(), GitProviderError>;
}

pub fn new(config: &Config) -> Result<Box<dyn GitProvider>, GitProviderError> {
    let git_cli = Git::init(config.format.clone());
    let repo = git_cli.get_repo_info().unwrap(); // TODO: use git::cli::Provider instead

    match repo.provider {
        Provider::Github => Ok(Box::new(github::Provider::new())),
        _ => Err(GitProviderError::ProviderNotSupported(repo.provider))
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
