use crate::dataprovider::git::GitDataProvider;
use crate::git::model::tag::Tag;
use crate::usecase::release::{Commit, GitDataProviderError, ReleaseDataProvider};

impl ReleaseDataProvider for GitDataProvider {
    fn find_latest_version(&self, app_name: &str) -> Result<Option<Tag>, GitDataProviderError> {
        let mut versions = self.git_client.get_tags(app_name)?;
        versions.sort_by(|a, b| b.cmp(&a));
        let version = versions
            .first()
            .map(|tag| tag.clone());

        Ok(version)
    }

    fn release(&self, name: &str, tag: &Tag, body: &String) -> Result<(), GitDataProviderError> {
        todo!()
    }

    fn get_commits(&self, tag: &Option<Tag>, path: Option<String>) -> Result<Vec<Commit>, GitDataProviderError> {
        todo!()
    }

    fn compare_url(&self, tag: &Option<Tag>, new_tag: &Tag) -> Result<Option<String>, GitDataProviderError> {
        todo!()
    }
}
