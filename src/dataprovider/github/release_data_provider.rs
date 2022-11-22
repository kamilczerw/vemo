use crate::dataprovider::github::GithubDataProvider;
use crate::git::model::tag::Tag;
use crate::usecase::release::{Commit, GitDataProviderError, ReleaseDataProvider};

impl ReleaseDataProvider for GithubDataProvider {
    fn find_latest_version(&self, app_name: &str) -> Result<Option<Tag>, GitDataProviderError> {
        todo!("Implement GithubDataProvider::find_latest_version")
    }

    fn release(&self, _name: &str, _tag: &Tag, _body: &String) -> Result<(), GitDataProviderError> {
        todo!("Implement GithubDataProvider::find_latest_version")
    }

    fn get_commits(&self, tag: &Option<Tag>, path: Option<String>) -> Result<Vec<Commit>, GitDataProviderError> {
        todo!("Implement GithubDataProvider::find_latest_version")
    }

    fn compare_url(&self, _tag: &Option<Tag>, _new_tag: &Tag) -> Result<Option<String>, GitDataProviderError> {
        todo!("Implement GithubDataProvider::find_latest_version")
    }
}
