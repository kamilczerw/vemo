use std::time::Duration;
use ureq::Agent;
use crate::dataprovider::git::GitClient;
use crate::git::model::tag::Tag;
use crate::usecase::release::{Commit, GitDataProviderError, ReleaseDataProvider};
use crate::dataprovider::git::GitDataProvider;

mod release_data_provider;
mod dataprovider;

#[cfg(test)] mod release_data_provider_test;
#[cfg(test)] mod test;

struct GithubDataProvider {
    git_client: GitDataProvider,
    http_client: Box<dyn HttpClient>
}

pub trait HttpClient {
    fn get(&self, url: &str) -> Result<String, HttpClientError>;
}

pub enum HttpClientError {
    UnexpectedError(String),
}

pub enum GithubDataProviderError {
    UnexpectedError(String),
}
