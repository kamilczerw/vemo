use mockall::mock;
use rstest::{fixture, rstest};
use semver::Version;
use crate::cfg::DEFAULT_TAG_FORMAT;
use crate::dataprovider::git::{GitClient as GitClientTrait, GitClientError, GitDataProvider};
use crate::git::model::tag::Tag;
use crate::usecase::release::{GitDataProviderError, ReleaseDataProvider};
use crate::usecase::release::test::fixtures::commit;
use crate::usecase::release::Commit;

pub(crate) mod fixtures;

mock!{
    pub GitClient {}

    impl GitClientTrait for GitClient {
        fn get_tags(&self, app_name: &str) -> Result<Vec<Tag>, GitClientError>;
        fn get_commits(&self, tag: &Option<Tag>, path: Option<String>) -> Result<Vec<Commit>, GitClientError>;
    }
}
