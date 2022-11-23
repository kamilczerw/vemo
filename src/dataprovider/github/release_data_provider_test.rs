use mockall::{mock, predicate};
use rstest::{fixture, rstest};
use semver::Version;

use crate::cfg::DEFAULT_TAG_FORMAT;
use crate::dataprovider::git::{GitClient as GitClientTrait, GitDataProvider};
use crate::dataprovider::git::GitClientError;
use crate::dataprovider::git::test::MockGitClient;
use crate::dataprovider::git::test::fixtures::git_client;
use crate::dataprovider::git::test::fixtures::tag;
use crate::dataprovider::github::{GithubDataProvider, GithubDataProviderError};
use crate::dataprovider::github::{HttpClient as HttpClientTrait, HttpClientError};
use crate::git::model::tag::Tag;
use crate::usecase::release::{GitDataProviderError, ReleaseDataProvider};
use crate::usecase::release::Commit;
use crate::usecase::release::test::fixtures::commit;
use ureq::Agent;
use crate::dataprovider::github::test::read_file;

mock!{
    pub HttpClient {}

    #[async_trait::async_trait]
    impl HttpClientTrait for HttpClient {
        async fn get(&self, url: &str) -> Result<String, HttpClientError>;
    }
}

#[fixture]
fn http() -> MockHttpClient {
    MockHttpClient::new()
}

fn commit_json(login: &str) -> String {
    format!(r#"{{"author": {{"login": "{}"}}}}"#, login)
}


#[rstest]
async fn when_getting_commit_author_then_commit_author_should_be_returned(
    mut git_client: MockGitClient,
    mut http: MockHttpClient,
) {
    let commit = commit("feat: add feature", "12163b7ef16ff917a1d55a59da89a812579d32b9", "kamilczerw");
    http
        .expect_get()
        .times(1)
        .returning(move |_| Ok(read_file("get_commit_response.json")));

    let git = GitDataProvider::new(Box::new(git_client));
    let provider = GithubDataProvider::new(git, Box::new(http));

    let result = provider.get_commit_author(&commit).await;

    assert!(result.is_ok());
    if let Ok(result) = result {
        assert_eq!(result, "kamilczerw");
    }
}

#[rstest]
async fn when_getting_commit_author_fails_then_commit_author_should_be_returned(
    mut git_client: MockGitClient,
    mut http: MockHttpClient,
) {
    let commit = commit("feat: add feature", "12163b7ef16ff917a1d55a59da89a812579d32b9", "kamilczerw");
    http
        .expect_get()
        .times(1)
        .returning(move |_| Err(HttpClientError::Unauthorized));

    let git = GitDataProvider::new(Box::new(git_client));
    let provider = GithubDataProvider::new(git, Box::new(http));

    let result = provider.get_commit_author(&commit).await;

    assert!(result.is_err());
    if let Err(GithubDataProviderError::Unauthorized) = result {
    } else { panic!("Expected error") }
}

#[rstest]
async fn when_getting_commits_author_should_be_fetched_in_background(
    mut git_client: MockGitClient,
    mut http: MockHttpClient,
) {
    let commits = vec![
        commit("mgs1", "hash1", "author1"),
        commit("mgs2", "hash2", "author2"),
    ];
    git_client
        .expect_get_commits()
        .returning(move |_, _| Ok(commits.clone()));
    http
        .expect_get()
        .with(predicate::function(|url: &str| url.contains("hash1")))
        .returning(move |_| Ok(commit_json("@author1")));

    http
        .expect_get()
        .with(predicate::function(|url: &str| url.contains("hash2")))
        .returning(move |_| Ok(commit_json("@author2")));

    let git = GitDataProvider::new(Box::new(git_client));
    let provider = GithubDataProvider::new(git, Box::new(http));

    let result = provider.get_commits(&Some(Tag::default()), None);

    assert!(result.is_ok());
    if let Ok(commits) = result {
        assert_eq!(commits.len(), 2);
        assert_eq!(commits[0].author, "@author1");
        assert_eq!(commits[1].author, "@author2");
    } else { panic!("Expected error") }
}

#[rstest]
async fn when_getting_compare_url_with_tag_then_correct_url_should_be_returned(
    mut git_client: MockGitClient,
    mut http: MockHttpClient,
) {
    let t1 = tag("1.0.0", "app");
    let t2 = tag("1.1.0", "app");
    let git = GitDataProvider::new(Box::new(git_client));
    let provider = GithubDataProvider::new(git, Box::new(http));

    let result = provider.compare_url(&Some(t1), &t2);

    assert!(result.is_ok());
    if let Ok(Some(url)) = result {
        assert_eq!(url, "https://github.com/abc/def/compare/app/v1.0.0...app/v1.1.0");
    } else { panic!("Did not expect an error") }
}

#[rstest]
async fn when_getting_compare_url_with_no_tag_then_correct_url_should_be_returned(
    mut git_client: MockGitClient,
    mut http: MockHttpClient,
) {
    let tag = tag("1.0.0", "app");
    let git = GitDataProvider::new(Box::new(git_client));
    let provider = GithubDataProvider::new(git, Box::new(http));

    let result = provider.compare_url(&None, &tag);

    assert!(result.is_ok());
    if let Ok(Some(url)) = result {
        assert_eq!(url, "https://github.com/abc/def/compare/main...app/v1.0.0");
    } else { panic!("Did not expect an error") }
}

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
