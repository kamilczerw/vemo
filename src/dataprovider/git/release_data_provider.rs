use semver::Version;
use crate::dataprovider::git::GitDataProvider;
use crate::git::model::tag::Tag;
use crate::usecase::release::{Commit, GitDataProviderError, ReleaseDataProvider};

impl ReleaseDataProvider for GitDataProvider {
    fn find_latest_version(&self, app_name: &str) -> Result<Option<Version>, GitDataProviderError> {
        let mut versions = self.git_client.get_tags(app_name)?;
        versions.sort_by(|a, b| a.version.cmp(&b.version));
        versions.reverse();
        let version = versions
            .first()
            .map(|tag| tag.version.clone());

        Ok(version)
    }

    fn release(&self, name: &str, tag: &Tag, body: &String) -> Result<(), GitDataProviderError> {
        todo!()
    }

    fn get_commits(&self, tag: &Tag, path: Option<String>) -> Result<Vec<Commit>, GitDataProviderError> {
        todo!()
    }

    fn compare_url(&self, tag: &Tag, new_tag: &Tag) -> Result<Option<String>, GitDataProviderError> {
        todo!()
    }
}
