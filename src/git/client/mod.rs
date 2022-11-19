use crate::git::git_provider::GitProvider;
use crate::{Config, git};
use crate::git::client::error::GitClientError;
use crate::git::GitClient;
use crate::git::model::repo::Repo;

mod github;
pub mod error;

/// Create a new GitClient
pub fn new_client(config: &Config, repo: Repo) -> Result<Box<dyn GitClient>, GitClientError> {
    match repo.provider {
        git::Provider::Github => github_client(config, repo),
        _ => Err(GitClientError::UnsupportedProvider(repo.provider))
    }
}

/// Create a new GithubClient
fn github_client(config: &Config, repo: Repo) -> Result<Box<dyn GitClient>, GitClientError> {
    config.gh_token.clone().map(|token| {
        github::GithubClient::new(token, repo).map(|client| Box::new(client) as Box<dyn GitClient>)
    }).ok_or(GitClientError::MissingToken(GitProvider::Github))?
}
