use crate::dataprovider::git::{GitClient, GitDataProvider};
use crate::git::model::repo::Repo;
use crate::usecase::release::GitDataProviderError;

impl GitDataProvider {
    pub fn new(git_client: Box<dyn GitClient>) -> GitDataProvider {
        Self { git_client }
    }

    pub fn get_repo(&self) -> Repo {
        self.git_client.get_repo()
    }
}
