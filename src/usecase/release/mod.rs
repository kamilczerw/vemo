mod usecase;

#[cfg(test)] mod test;
#[cfg(test)] mod release_test;

use semver::Version;
use crate::cfg::AppConfig;
use crate::git::model::tag::Tag;
use crate::usecase::UseCase;

pub const DEFAULT_VERSION: Version = Version::new(0, 1, 0);

pub struct AppReleaseUseCase {
    pub(crate) release_data_provider: Box<dyn ReleaseDataProvider>,
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

pub trait ReleaseDataProvider {
    fn find_latest_version(&self, app_name: &str) -> Result<Option<Version>, GitDataProviderError>;
    fn release(&self, name: &str, tag: &Tag, body: &String) -> Result<(), GitDataProviderError>;
    fn get_commits(&self, tag: &Tag, path: Option<String>) -> Result<Vec<Commit>, GitDataProviderError>;
    fn compare_url(&self, tag: &Tag, new_tag: &Tag) -> Result<Option<String>, GitDataProviderError>;
}

pub trait ConfigDataProvider {
    fn get_app_config(&self, app_name: &str) -> Result<AppConfig, ConfigDataProviderError>;
}

#[derive(Eq, PartialEq, Debug)]
pub enum AppReleaseUseCaseError {
    UnexpectedError(String),
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
    UnexpectedError(String),
}

impl From<GitDataProviderError> for AppReleaseUseCaseError {
    fn from(error: GitDataProviderError) -> Self {
        match error {
            GitDataProviderError::UnexpectedError(msg) => AppReleaseUseCaseError::UnexpectedError(msg)
        }
    }
}

pub enum ConfigDataProviderError {
    UnexpectedError(String),
}

impl From<ConfigDataProviderError> for AppReleaseUseCaseError {
    fn from(error: ConfigDataProviderError) -> Self {
        match error {
            ConfigDataProviderError::UnexpectedError(msg) => AppReleaseUseCaseError::UnexpectedError(msg)
        }
    }
}
