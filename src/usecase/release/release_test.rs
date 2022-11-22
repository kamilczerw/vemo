use crate::usecase::UseCase;
use mockall::predicate::*;
use rstest::rstest;
use semver::Version;
use crate::cfg;
use crate::git::model::tag::Tag;
use crate::usecase::release::{AppReleaseUseCaseError, Commit, Component, GitDataProviderError, MockGitDataProvider};
use crate::usecase::release::MockConfigDataProvider;
use crate::usecase::release::{AppReleaseUseCase, AppReleaseUseCaseRequest, ConfigDataProvider, GitDataProvider};
use crate::usecase::release::test::fixtures::*;

#[rstest]
fn when_app_name_and_component_are_provided_and_there_are_no_commits_then_version_should_be_bumped(
    mut release_provider: MockGitDataProvider,
    mut app_config: MockConfigDataProvider
) {
    release_provider.expect_release().times(0);
    release_provider.expect_get_commits().times(1).returning(|_, _| Ok(vec![]));

    let use_case = use_case(release_provider, app_config);

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
    mut app_config: MockConfigDataProvider
) {
    provider_with_commits.expect_release().times(1).returning(|_, _, _| Ok(()));
    provider_with_commits.expect_compare_url().times(1).returning(|t1, t2| Ok(Some(format!("https://github.com/bla/{}/compare/{}...{}", APP_NAME, t1, t2))));

    let use_case = use_case(provider_with_commits, app_config);

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
    mut app_config: MockConfigDataProvider
) {
    provider_with_commits.expect_release().times(1).returning(|_, _, _| Ok(()));
    provider_with_commits.expect_compare_url().times(1).returning(|_, _| Ok(None));

    let use_case = use_case(provider_with_commits, app_config);

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

#[rstest]
fn when_there_is_no_path_in_app_config_the_path_should_not_be_passed(
    mut provider_with_commits: MockGitDataProvider,
    mut config: MockConfigDataProvider
) {
    provider_with_commits.expect_release().times(1).returning(|_, _, _| Ok(()));
    provider_with_commits.expect_compare_url().times(1).returning(|_, _| Ok(None));
    config.expect_get_app_config().times(1).returning(|_| Ok(cfg::AppConfig { path: None }));

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

#[rstest]
fn when_find_latest_tag_fails_then_a_failure_should_be_returned(
    mut empty_provider: MockGitDataProvider,
    mut config: MockConfigDataProvider
) {
    empty_provider.expect_find_latest_version().times(1)
        .returning(|_| Err(GitDataProviderError::UnexpectedError("Failed to get latest version".to_string())));

    let use_case = use_case(empty_provider, config);

    let request = AppReleaseUseCaseRequest {
        app_name: APP_NAME.to_string(),
        component: Component::Patch
    };

    if let Err(AppReleaseUseCaseError::UnexpectedError(message)) = use_case.execute(request) {
        assert_eq!(message, "Failed to get latest version");
    } else {
        panic!("Expected error");
    }
}
