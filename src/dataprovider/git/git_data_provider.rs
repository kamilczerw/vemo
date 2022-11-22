use crate::dataprovider::git::{GitClient, GitDataProvider};

impl GitDataProvider {
    pub fn new(git_client: Box<dyn GitClient>) -> GitDataProvider {
        Self { git_client }
    }
}
