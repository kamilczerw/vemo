mod usecase;

#[cfg(test)] mod test;
#[cfg(test)] mod release_test;

use semver::Version;
use mockall::automock;
use crate::cfg::AppConfig;
use crate::git::model::tag::Tag;
use crate::usecase::UseCase;

pub struct AppReleaseUseCase {
    pub(crate) git_provider: Box<dyn GitDataProvider>,
    pub(crate) config_data_provider: Box<dyn ConfigDataProvider>,
    pub(crate) format: String,
}


pub struct AppReleaseUseCaseRequest {
    pub(crate) app_name: String,
    pub(crate) component: Component
}

pub struct AppReleaseUseCaseResponse {
    pub(crate) tag: Tag,
    pub(crate) body: String
}

#[automock]
pub trait GitDataProvider {
    fn find_latest_version(&self, app_name: &str) -> Result<Option<Version>, GitDataProviderError>;
    fn release(&self, name: &str, tag: &Tag, body: &String) -> Result<(), GitDataProviderError>;
    fn get_commits(&self, tag: &Tag, path: Option<String>) -> Result<Vec<Commit>, GitDataProviderError>;
    fn compare_url(&self, tag: &Tag, new_tag: &Tag) -> Option<String>;
}

#[automock]
pub trait ConfigDataProvider {
    fn get_app_config(&self, app_name: &str) -> Result<AppConfig, ConfigDataProviderError>;
}

#[derive(Eq, PartialEq, Debug)]
pub enum AppReleaseUseCaseError {
    UnexpectedError,
    NoChanges
}

pub enum Component {
    Major,
    Minor,
    Patch
}

pub struct Commit {
    pub hash: String,
    pub message: String,
    pub author: String, // TODO: change to author object
}

pub enum GitDataProviderError {
    UnexpectedError
}

impl From<GitDataProviderError> for AppReleaseUseCaseError {
    fn from(error: GitDataProviderError) -> Self {
        match error {
            GitDataProviderError::UnexpectedError => AppReleaseUseCaseError::UnexpectedError
        }
    }
}

pub enum ConfigDataProviderError {
    UnexpectedError
}

impl From<ConfigDataProviderError> for AppReleaseUseCaseError {
    fn from(error: ConfigDataProviderError) -> Self {
        match error {
            ConfigDataProviderError::UnexpectedError => AppReleaseUseCaseError::UnexpectedError
        }
    }
}
