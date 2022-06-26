use crate::commands::shell::git::GitProvider;
use crate::Config;
use crate::git::client::error::GitClientError;
use crate::git::GitClient;
use crate::git::model::Repo;

mod github;
pub mod error;
mod local;

/// Create a new GitClient
pub fn new_client(config: &Config, repo: Repo) -> Result<Box<dyn GitClient>, GitClientError> {
    match repo.provider {
        GitProvider::Github => github_client(config, repo),
        _ => local_client(config, repo)
    }
}

/// Create a new GithubClient
fn github_client(config: &Config, repo: Repo) -> Result<Box<dyn GitClient>, GitClientError> {
    config.gh_token.clone().map(|token| {
        github::GithubClient::new(token, repo).map(|client| Box::new(client) as Box<dyn GitClient>)
    }).ok_or(GitClientError::MissingToken(GitProvider::Github))?
}

/// Create a local GitClient
fn local_client(config: &Config, repo: Repo) -> Result<Box<dyn GitClient>, GitClientError> {
    Ok(Box::new(local::LocalClient::init(config.format.clone())))
}
