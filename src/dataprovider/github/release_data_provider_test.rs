use mockall::mock;
use rstest::{fixture, rstest};
use semver::Version;
use crate::cfg::DEFAULT_TAG_FORMAT;
use crate::dataprovider::github::{GithubClient as GithubClientTrait, GithubClientError, GithubDataProvider};
use crate::dataprovider::git::test::{MockGitClient};
use crate::dataprovider::git::test::fixtures::tag;
use crate::dataprovider::git::test::fixtures::git_client;
use crate::dataprovider::git::GitClient as GitClientTrait;
use crate::git::model::tag::Tag;
use crate::usecase::release::{GitDataProviderError, ReleaseDataProvider};
use crate::usecase::release::test::fixtures::commit;
use crate::usecase::release::Commit;
use crate::dataprovider::git::GitClientError;

mock!{
    pub GithubClient {}

    impl GithubClientTrait for GithubClient {
    }

    impl GitClientTrait for GithubClient {
        fn get_tags(&self, app_name: &str) -> Result<Vec<Tag>, GitClientError>;
        fn get_commits(&self, tag: &Option<Tag>, path: Option<String>) -> Result<Vec<Commit>, GitClientError>;
    }
}


#[fixture]
fn github_client() -> MockGithubClient {
    MockGithubClient::new()
}

// #[rstest]
// fn when_getting_latest_version_and_no_version_exists_then_return_none(
//     mut git_client: MockGitClient,
//     mut github_client: MockGithubClient,
// ) {
//     git_client
//         .expect_get_tags()
//         .times(1)
//         .returning(|_| Ok(vec![]));
//
//     let provider = GithubDataProvider::new(Box::new(git_client), Box::new(github_client));
//
//     let result = provider.find_latest_version("app");
//
//     assert!(result.is_ok());
//     if let Ok(result) = result {
//         assert!(result.is_none());
//     }
// }
//
// #[rstest]
// fn when_getting_latest_version_and_version_exists_then_return_version(
//     mut git_client: MockGitClient,
//     mut github_client: MockGithubClient,
// ) {
//     git_client
//         .expect_get_tags()
//         .times(1)
//         .returning(|_| Ok(vec![tag("2.0.0", "app"), tag("1.0.0", "app")]));
//
//     let provider = GithubDataProvider::new(Box::new(git_client), Box::new(github_client));
//
//     let result = provider.find_latest_version("app");
//
//     assert!(result.is_ok());
//     if let Ok(Some(tag)) = result {
//         assert_eq!(tag.version, Version::parse("2.0.0").unwrap());
//     } else {
//         panic!("Expected version to be Some");
//     }
// }
//
// #[rstest]
// fn when_getting_latest_version_and_git_client_fails_then_return_failure(
//     mut git_client: MockGitClient,
//     mut github_client: MockGithubClient,
// ) {
//     git_client
//         .expect_get_tags()
//         .times(1)
//         .returning(|_| Err(GithubClientError::UnexpectedError("Unexpected error".to_string())));
//
//     let provider = GithubDataProvider::new(Box::new(git_client), Box::new(github_client));
//
//     let result = provider.find_latest_version("app");
//
//     assert!(result.is_err());
//     if let Err(GitDataProviderError::UnexpectedError(message)) = result {
//         assert_eq!(message, "Unexpected error");
//     } else {
//         panic!("Expected error to be UnexpectedError");
//     }
// }
//
// #[rstest]
// fn when_getting_commits_then_commits_should_be_returned(
//     mut git_client: MockGithubClient,
// ) {
//     git_client
//         .expect_get_commits()
//         .times(1)
//         .returning(|_, _| Ok( vec![
//             commit("msg1", "hash1", "author1")
//         ]));
//
//     let provider = GithubDataProvider::new(Box::new(git_client));
//
//     let result = provider.get_commits(&Some(tag("1.0.0", "app")), Some("app".to_string()));
//
//     assert!(result.is_ok());
//     if let Ok(commits) = result {
//         assert_eq!(commits.len(), 1);
//     } else {
//         panic!("Expected error to be UnexpectedError");
//     }
// }
