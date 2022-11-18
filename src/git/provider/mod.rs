use crate::git::git_provider;
use crate::git::model::repo::Repo;
use crate::git::model::tag::Tag;
use crate::git::provider::error::GitProviderError;

pub mod error;

mod github;

pub trait GitProvider {
    fn find_latest_tag(&self, app_name: &str) -> Result<Option<Tag>, GitProviderError>;

    fn release(&self, name: &str, tag: Tag, body: String) -> Result<(), GitProviderError>;
}

pub fn new(repo: &Repo) -> Result<Box<dyn GitProvider>, GitProviderError> {
    match repo.provider {
        git_provider::GitProvider::Github => Ok(Box::new(github::Provider::new())),
        _ => Err(GitProviderError::ProviderNotSupported(repo.provider.clone().to_string()))
    }
}
