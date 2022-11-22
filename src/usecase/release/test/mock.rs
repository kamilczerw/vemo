use mockall::mock;
use crate::cfg::AppConfig;
use crate::git::model::tag::Tag;
use crate::usecase::release::{Commit, ConfigDataProvider as ConfigDataProviderTrait, ConfigDataProviderError, ReleaseDataProvider as GitDataProviderTrait, GitDataProviderError};

mock!{
    pub ReleaseDataProvider {}

    impl GitDataProviderTrait for ReleaseDataProvider {
        fn find_latest_version(&self, app_name: &str) -> Result<Option<Tag>, GitDataProviderError>;
        fn release(&self, name: &str, tag: &Tag, body: &String) -> Result<(), GitDataProviderError>;
        fn get_commits(&self, tag: &Option<Tag>, path: Option<String>) -> Result<Vec<Commit>, GitDataProviderError>;
        fn compare_url(&self, tag: &Option<Tag>, new_tag: &Tag) -> Result<Option<String>, GitDataProviderError>;
    }
}

mock!{
    pub ConfigDataProvider {}

    impl ConfigDataProviderTrait for ConfigDataProvider {
        fn get_app_config(&self, app_name: &str) -> Result<AppConfig, ConfigDataProviderError>;
    }
}
