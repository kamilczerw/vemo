use mockall::mock;
use rstest::{fixture, rstest};
use semver::Version;
use crate::cfg::DEFAULT_TAG_FORMAT;
use crate::dataprovider::git::{GitClient as GitClientTrait, GitClientError, GitDataProvider};
use crate::git::model::tag::Tag;
use crate::usecase::release::ReleaseDataProvider;

mock!{
    pub GitClient {}

    impl GitClientTrait for GitClient {
        fn get_tags(&self, app_name: &str) -> Result<Vec<Tag>, GitClientError>;
    }
}

fn tag(version: &str, app_name: &str) -> Tag {
    Tag {
        format: DEFAULT_TAG_FORMAT.to_string(),
        raw: format!("{}-{}", app_name, version),
        version: Version::parse(version).unwrap(),
        app_name: app_name.to_string()
    }
}

#[fixture]
fn git_client() -> MockGitClient {
    MockGitClient::new()
}

#[rstest]
fn when_getting_latest_version_and_no_version_exists_then_return_none(
    mut git_client: MockGitClient,
) {
    git_client
        .expect_get_tags()
        .times(1)
        .returning(|_| Ok(vec![]));

    let provider = GitDataProvider::new(Box::new(git_client));

    let result = provider.find_latest_version("app");

    assert!(result.is_ok());
    if let Ok(result) = result {
        assert!(result.is_none());
    }
}

#[rstest]
fn when_getting_latest_version_and_version_exists_then_return_version(
    mut git_client: MockGitClient,
) {
    git_client
        .expect_get_tags()
        .times(1)
        .returning(|_| Ok(vec![tag("2.0.0", "app"), tag("1.0.0", "app")]));

    let provider = GitDataProvider::new(Box::new(git_client));

    let result = provider.find_latest_version("app");

    assert!(result.is_ok());
    if let Ok(Some(version)) = result {
        assert_eq!(version, Version::parse("2.0.0").unwrap());
    } else {
        panic!("Expected version to be Some");
    }
}
