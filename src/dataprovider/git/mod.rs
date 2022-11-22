use crate::git::model::repo::Repo;
use crate::git::model::tag::Tag;
use crate::usecase::release::{Commit, GitDataProviderError};

mod release_data_provider;
mod git_data_provider;

#[cfg(test)] mod release_data_provider_test;
#[cfg(test)]
pub(crate) mod test;


pub(crate) struct GitDataProvider {
    git_client: Box<dyn GitClient>
}

pub(crate) trait GitClient {
    fn get_tags(&self, app_name: &str) -> Result<Vec<Tag>, GitClientError>;
    fn get_commits(&self, tag: &Option<Tag>, path: Option<String>) -> Result<Vec<Commit>, GitClientError>;
    fn get_repo(&self) -> Repo;
}

pub enum GitClientError {
    UnexpectedError(String),
}

impl From<GitClientError> for GitDataProviderError {
    fn from(error: GitClientError) -> Self {
        match error {
            GitClientError::UnexpectedError(message) => GitDataProviderError::UnexpectedError(message)
        }
    }
}
