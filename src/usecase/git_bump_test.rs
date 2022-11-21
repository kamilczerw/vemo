use crate::usecase::git_bump::{Component, AppReleaseUseCase, AppReleaseUseCaseRequest, AppReleaseUseCaseError, GitDataProvider, GitDataProviderError, ConfigDataProvider, Commit};
use crate::usecase::UseCase;
use mockall::predicate::*;
use rstest::{fixture, rstest};
use semver::Version;
use crate::git::model::tag::Tag;
use crate::usecase::git_bump::MockGitDataProvider;
use crate::usecase::git_bump::MockConfigDataProvider;

const APP_NAME: &str = "app";
const FORMAT: &str = "v{version}";

// fn setup_git_provider() -> MockGitDataProvider {
//     let mut git_provider = MockGitDataProvider::new();
//     git_provider
//         .expect_find_latest_version()
//         .with(eq(APP_NAME))
//         .times(1)
//         .returning(|_| Ok(Some(Version::new(1, 2, 3))));
//     // git_provider
//     //     .expect_release()
//     //     .with(eq(APP_NAME), eq(Tag::new_with_format(FORMAT, APP_NAME, Version::new(1, 2, 4))), eq(String::from("## What's Changed\n\n")))
//     //     .times(1)
//     //     .returning(|_, _, _| Ok(()));
//
//     git_provider
// }

fn use_case(provider: MockGitDataProvider, config: MockConfigDataProvider) -> AppReleaseUseCase {
    AppReleaseUseCase {
        git_provider: Box::new(provider),
        config_data_provider: Box::new(config),
        format: String::from(FORMAT)
    }
}

fn commit(message: &str, hash: &str, author: &str) -> Commit {
    Commit {
        message: String::from(message),
        hash: String::from(hash),
        author: String::from(author),
    }
}


#[fixture]
fn empty_provider() -> MockGitDataProvider {
    MockGitDataProvider::new()
}

#[fixture]
fn release_provider(mut empty_provider: MockGitDataProvider) -> MockGitDataProvider {
    empty_provider
        .expect_find_latest_version()
        .with(eq(APP_NAME))
        .times(1)
        .returning(|_| Ok(Some(Version::new(1, 2, 3))));

    empty_provider
}

#[fixture]
fn provider_with_commits(mut release_provider: MockGitDataProvider) -> MockGitDataProvider {
    release_provider
        .expect_get_commits()
        // .with(eq(APP_NAME), eq("path"))
        .times(1)
        .returning(|_, _| Ok( vec![
            commit("feat: add feature", "hash1", "author1"),
            commit("fix: fix bug", "hash2", "author2"),
            commit("chore: update dependencies", "hash3", "author3"),
        ]));

    release_provider
}

#[fixture]
fn config() -> MockConfigDataProvider {
    MockConfigDataProvider::new()
}


#[rstest]
fn when_app_name_and_component_are_provided_and_there_are_no_commits_then_version_should_be_bumped(
    mut release_provider: MockGitDataProvider,
    mut config: MockConfigDataProvider
) {
    release_provider.expect_release().times(0);
    release_provider.expect_get_commits().times(1).returning(|_, _| Ok(vec![]));

    let use_case = use_case(release_provider, config);

    let request = AppReleaseUseCaseRequest {
        app_name: APP_NAME.to_string(),
        component: Component::Patch
    };

    let result = use_case.execute(request);
    assert!(result.is_err());

    if let Err(error) = result {
        assert_eq!(error, AppReleaseUseCaseError::NoChanges);
    } else {
        panic!("Expected error");
    }
}

#[rstest]
fn when_app_name_and_component_are_provided_and_there_are_commits_then_version_should_be_bumped(
    mut provider_with_commits: MockGitDataProvider,
    mut config: MockConfigDataProvider
) {
    provider_with_commits.expect_release().times(1).returning(|_, _, _| Ok(()));
    provider_with_commits.expect_compare_url().times(1).returning(|t1, t2| Some(format!("https://github.com/bla/{}/compare/{}...{}", APP_NAME, t1, t2)));

    let use_case = use_case(provider_with_commits, config);

    let request = AppReleaseUseCaseRequest {
        app_name: APP_NAME.to_string(),
        component: Component::Patch
    };

    let mut expected_message = String::from("## What's Changed\n\n");
    expected_message.push_str("* feat: add feature by author1\n");
    expected_message.push_str("* fix: fix bug by author2\n");
    expected_message.push_str("* chore: update dependencies by author3\n");
    expected_message.push_str("\n\n");
    expected_message.push_str(&format!("**Full Changelog**: https://github.com/bla/{}/compare/{}...{}", APP_NAME, "v1.2.3", "v1.2.4"));

    let result = use_case.execute(request).unwrap();
    assert_eq!(result.tag, Tag::new_with_format(FORMAT, APP_NAME, Version::new(1, 2, 4)));
    assert_eq!(result.body, expected_message);
}

#[rstest]
fn when_app_name_and_component_are_provided_and_there_are_commits_but_no_compare_url_then_body_should_not_contain_compare_url(
    mut provider_with_commits: MockGitDataProvider,
    mut config: MockConfigDataProvider
) {
    provider_with_commits.expect_release().times(1).returning(|_, _, _| Ok(()));
    provider_with_commits.expect_compare_url().times(1).returning(|_, _| None);

    let use_case = use_case(provider_with_commits, config);

    let request = AppReleaseUseCaseRequest {
        app_name: APP_NAME.to_string(),
        component: Component::Patch
    };

    let mut expected_message = String::from("## What's Changed\n\n");
    expected_message.push_str("* feat: add feature by author1\n");
    expected_message.push_str("* fix: fix bug by author2\n");
    expected_message.push_str("* chore: update dependencies by author3\n");

    let result = use_case.execute(request).unwrap();
    assert_eq!(result.tag, Tag::new_with_format(FORMAT, APP_NAME, Version::new(1, 2, 4)));
    assert_eq!(result.body, expected_message);
}
