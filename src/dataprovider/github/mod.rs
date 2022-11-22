use std::time::Duration;
use ureq::Agent;
use crate::dataprovider::git::GitClient;
use crate::git::model::tag::Tag;
use crate::usecase::release::{Commit, GitDataProviderError, ReleaseDataProvider};
use crate::dataprovider::git::GitDataProvider;
use async_trait::async_trait;


mod release_data_provider;
mod dataprovider;

#[cfg(test)] mod release_data_provider_test;
#[cfg(test)] mod test;

struct GithubDataProvider {
    git_client: GitDataProvider,
    http_client: Box<dyn HttpClient>,
    github_api_url: String,
}

#[async_trait]
pub trait HttpClient {
    async fn get(&self, url: &str) -> Result<String, HttpClientError>;
}

pub enum HttpClientError {
    UnexpectedError(String),
    Unauthorized,
}

pub enum GithubDataProviderError {
    UnexpectedError(String),
    Unauthorized,
}

impl From<HttpClientError> for GithubDataProviderError {
    fn from(error: HttpClientError) -> Self {
        match error {
            HttpClientError::UnexpectedError(message) => GithubDataProviderError::UnexpectedError(message),
            HttpClientError::Unauthorized => GithubDataProviderError::Unauthorized,
        }
    }
}
