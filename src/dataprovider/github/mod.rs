use crate::dataprovider::git::GitClient;
use crate::git::model::tag::Tag;
use crate::usecase::release::{Commit, GitDataProviderError};

mod release_data_provider;

#[cfg(test)] mod release_data_provider_test;

struct GithubDataProvider {
    git_client: Box<dyn GitClient>,
    github_client: Box<dyn GithubClient>
}

// impl GithubDataProvider {
//     pub fn new(git: Box<dyn GitClient>, github: Box<dyn GithubClient>) -> GithubDataProvider {
//         Self { git_client: git, github_client: github }
//     }
// }

impl GithubDataProvider {
    pub fn new(git_client: Box<dyn GitClient>, github: Box<dyn GithubClient>) -> GithubDataProvider {
        Self { git_client, github_client: github }
    }
}

trait GithubClient: GitClient {
    // fn get_tags(&self, app_name: &str) -> Result<Vec<Tag>, GithubClientError>;
    // fn get_commits(&self, tag: &Option<Tag>, path: Option<String>) -> Result<Vec<Commit>, GithubClientError>;
}

pub enum GithubClientError {
    UnexpectedError(String),
}

impl From<GithubClientError> for GitDataProviderError {
    fn from(error: GithubClientError) -> Self {
        match error {
            GithubClientError::UnexpectedError(message) => GitDataProviderError::UnexpectedError(message)
        }
    }
}
