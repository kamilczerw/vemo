use mockall::mock;
use semver::Version;
use crate::cfg::AppConfig;
use crate::git::model::tag::Tag;
use crate::usecase::release::{Commit, ConfigDataProvider as ConfigDataProviderTrait, ConfigDataProviderError, GitDataProvider as GitDataProviderTrait, GitDataProviderError};

mock!{
    pub ReleaseDataProvider {}

    impl GitDataProviderTrait for ReleaseDataProvider {
        fn find_latest_version(&self, app_name: &str) -> Result<Option<Version>, GitDataProviderError>;
        fn release(&self, name: &str, tag: &Tag, body: &String) -> Result<(), GitDataProviderError>;
        fn get_commits(&self, tag: &Tag, path: Option<String>) -> Result<Vec<Commit>, GitDataProviderError>;
        fn compare_url(&self, tag: &Tag, new_tag: &Tag) -> Result<Option<String>, GitDataProviderError>;
    }
}

mock!{
    pub ConfigDataProvider {}

    impl ConfigDataProviderTrait for ConfigDataProvider {
        fn get_app_config(&self, app_name: &str) -> Result<AppConfig, ConfigDataProviderError>;
    }
}
