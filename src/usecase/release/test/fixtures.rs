use mockall::predicate::eq;
use rstest::fixture;
use semver::Version;

use crate::cfg;
use crate::git::model::tag::Tag;
use crate::usecase::release::{AppReleaseUseCase, Commit};
use crate::usecase::release::test::mock::{MockConfigDataProvider, MockReleaseDataProvider};

pub const APP_NAME: &str = "app";
pub const FORMAT: &str = "v{version}";

pub fn default_tag() -> Tag {
    Tag::new_with_format(FORMAT, APP_NAME, Version::new(1, 2, 3))
}

#[fixture]
pub fn empty_provider() -> MockReleaseDataProvider {
    MockReleaseDataProvider::new()
}

#[fixture]
pub fn release_provider(mut empty_provider: MockReleaseDataProvider) -> MockReleaseDataProvider {
    empty_provider
        .expect_find_latest_tag()
        .with(eq(APP_NAME))
        .returning(|_| Ok(Some(default_tag())));

    empty_provider
}

#[fixture]
pub fn provider_with_commits(mut release_provider: MockReleaseDataProvider) -> MockReleaseDataProvider {
    release_provider
        .expect_get_commits()
        // .with(eq(default_tag()), eq(Some(String::from("path"))))
        .returning(|_, _| Ok( vec![
            commit("feat: add feature", "hash1", "author1"),
            commit("fix: fix bug", "hash2", "author2"),
            commit("chore: update dependencies", "hash3", "author3"),
        ]));

    release_provider
}

#[fixture]
pub fn config() -> MockConfigDataProvider {
    MockConfigDataProvider::new()
}

#[fixture]
pub fn app_config(mut config: MockConfigDataProvider) -> MockConfigDataProvider {
    config
        .expect_get_app_config()
        .times(1)
        .returning(|_| Ok(cfg::AppConfig {
            path: Some(String::from("path")),
        }));

    config
}


pub fn use_case(provider: MockReleaseDataProvider, config: MockConfigDataProvider) -> AppReleaseUseCase {
    AppReleaseUseCase {
        release_data_provider: Box::new(provider),
        config_data_provider: Box::new(config),
        format: String::from(FORMAT)
    }
}

pub fn commit(message: &str, hash: &str, author: &str) -> Commit {
    Commit {
        message: String::from(message),
        hash: String::from(hash),
        author: String::from(author),
    }
}
