use mockall::mock;
use rstest::{fixture, rstest};
use semver::Version;
use crate::cfg::DEFAULT_TAG_FORMAT;
use crate::dataprovider::git::{GitClient as GitClientTrait, GitClientError, GitDataProvider};
use crate::git::model::tag::Tag;
use crate::usecase::release::{GitDataProviderError, ReleaseDataProvider};
use crate::usecase::release::test::fixtures::commit;
use crate::usecase::release::Commit;
use crate::dataprovider::git::test::{fixtures, MockGitClient};
use crate::dataprovider::git::test::fixtures::git_client;

#[rstest]
fn when_getting_latest_version_and_no_version_exists_then_return_none(
    mut git_client: MockGitClient,
) {
    git_client
        .expect_get_tags()
        .times(1)
        .returning(|_| Ok(vec![]));

    let provider = GitDataProvider::new(Box::new(git_client));

    let result = provider.find_latest_tag("app");

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
        .returning(|_| Ok(vec![fixtures::tag("2.0.0", "app"), fixtures::tag("1.0.0", "app")]));

    let provider = GitDataProvider::new(Box::new(git_client));

    let result = provider.find_latest_tag("app");

    assert!(result.is_ok());
    if let Ok(Some(tag)) = result {
        assert_eq!(tag.version, Version::parse("2.0.0").unwrap());
    } else {
        panic!("Expected version to be Some");
    }
}

#[rstest]
fn when_getting_latest_version_and_git_client_fails_then_return_failure(
    mut git_client: MockGitClient,
) {
    git_client
        .expect_get_tags()
        .times(1)
        .returning(|_| Err(GitClientError::UnexpectedError("Unexpected error".to_string())));

    let provider = GitDataProvider::new(Box::new(git_client));

    let result = provider.find_latest_tag("app");

    assert!(result.is_err());
    if let Err(GitDataProviderError::UnexpectedError(message)) = result {
        assert_eq!(message, "Unexpected error");
    } else {
        panic!("Expected error to be UnexpectedError");
    }
}

#[rstest]
fn when_getting_commits_then_commits_should_be_returned(
    mut git_client: MockGitClient,
) {
    git_client
        .expect_get_commits()
        .times(1)
        .returning(|_, _| Ok( vec![
            commit("msg1", "hash1", "author1")
        ]));

    let provider = GitDataProvider::new(Box::new(git_client));

    let result = provider.get_commits(&Some(fixtures::tag("1.0.0", "app")), Some("app".to_string()));

    assert!(result.is_ok());
    if let Ok(commits) = result {
        assert_eq!(commits.len(), 1);
    } else {
        panic!("Expected error to be UnexpectedError");
    }
}
