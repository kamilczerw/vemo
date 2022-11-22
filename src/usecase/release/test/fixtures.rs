use mockall::predicate::eq;
use semver::Version;
use rstest::{fixture, rstest};
use crate::usecase::release::{AppReleaseUseCase, AppReleaseUseCaseError, Commit, Component};
use crate::cfg;
use crate::usecase::release::test::mock::{MockReleaseDataProvider, MockConfigDataProvider};

pub const APP_NAME: &str = "app";
pub const FORMAT: &str = "v{version}";

#[fixture]
pub fn empty_provider() -> MockReleaseDataProvider {
    MockReleaseDataProvider::new()
}

#[fixture]
pub fn release_provider(mut empty_provider: MockReleaseDataProvider) -> MockReleaseDataProvider {
    empty_provider
        .expect_find_latest_version()
        .with(eq(APP_NAME))
        .times(1)
        .returning(|_| Ok(Some(Version::new(1, 2, 3))));

    empty_provider
}

#[fixture]
pub fn provider_with_commits(mut release_provider: MockReleaseDataProvider) -> MockReleaseDataProvider {
    release_provider
        .expect_get_commits()
        // .with(eq(Tag::new_with_format(FORMAT, APP_NAME, Version::new(1, 2, 4))), eq(Some(String::from("path"))))
        .times(1)
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
        git_provider: Box::new(provider),
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
