use rstest::rstest;
use semver::Version;

use crate::cfg;
use crate::git::model::tag::Tag;
use crate::usecase::release::{AppReleaseUseCaseError, Component, ConfigDataProviderError, GitDataProviderError};
use crate::usecase::release::AppReleaseUseCaseRequest;
use crate::usecase::release::test::fixtures::*;
use crate::usecase::release::test::mock::{MockConfigDataProvider, MockReleaseDataProvider};
use crate::usecase::UseCase;

#[rstest]
fn when_app_name_and_component_are_provided_and_there_are_no_commits_then_version_should_be_bumped(
    mut release_provider: MockReleaseDataProvider,
    app_config: MockConfigDataProvider
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
    mut provider_with_commits: MockReleaseDataProvider,
    app_config: MockConfigDataProvider
) {
    provider_with_commits.expect_release().times(1).returning(|_, _, _| Ok(()));
    provider_with_commits.expect_compare_url().times(1)
        .returning(|t1, t2| Ok(Some(format!("https://github.com/bla/{}/compare/{}...{}", APP_NAME, t1.as_ref().unwrap(), t2))));

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
    mut provider_with_commits: MockReleaseDataProvider,
    app_config: MockConfigDataProvider
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
    mut provider_with_commits: MockReleaseDataProvider,
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
    mut empty_provider: MockReleaseDataProvider,
    config: MockConfigDataProvider
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

#[rstest]
fn when_find_latest_tag_returns_no_version_then_a_default_version_should_be_used(
    mut provider_with_commits: MockReleaseDataProvider,
    app_config: MockConfigDataProvider
) {
    provider_with_commits.expect_find_latest_version().times(1).returning(|_| Ok(None));
    provider_with_commits.expect_compare_url().times(1).returning(|_, _| Ok(None));
    provider_with_commits.expect_release().times(1).returning(|_, _, _| Ok(()));

    let use_case = use_case(provider_with_commits, app_config);

    let request = AppReleaseUseCaseRequest {
        app_name: "no-version".to_string(),
        component: Component::Patch
    };

    if let Ok(response) = use_case.execute(request) {
        assert_eq!(response.tag.to_string(), "v0.1.0");
    } else {
        panic!("Should not fail");
    }
}

#[rstest]
fn when_getting_config_fails_then_unexpected_error_should_be_returned(
    mut provider_with_commits: MockReleaseDataProvider,
    mut config: MockConfigDataProvider
) {
    config.expect_get_app_config().times(1)
        .returning(|_| Err(ConfigDataProviderError::UnexpectedError("Failed to get config".to_string())));
    provider_with_commits.expect_find_latest_version().times(1).returning(|_| Ok(None));

    let use_case = use_case(provider_with_commits, config);

    let request = AppReleaseUseCaseRequest {
        app_name: "no-version".to_string(),
        component: Component::Patch
    };

    if let Err(AppReleaseUseCaseError::UnexpectedError(message)) = use_case.execute(request) {
        assert_eq!(message, "Failed to get config");
    } else {
        panic!("Expected error");
    }
}
