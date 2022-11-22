use crate::git::model::tag::Tag;
use crate::usecase::release::GitDataProviderError;

mod release_data_provider;

#[cfg(test)] mod release_data_provider_test;
mod git_data_provider;

struct GitDataProvider {
    git_client: Box<dyn GitClient>
}

trait GitClient {
    fn get_tags(&self, app_name: &str) -> Result<Vec<Tag>, GitClientError>;
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
