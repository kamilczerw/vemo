use crate::usecase::git_bump::{Component, AppReleaseUseCase, AppReleaseUseCaseRequest, AppReleaseUseCaseError};
use crate::usecase::UseCase;
use mockall::predicate::*;
use semver::Version;
use crate::git::model::tag::Tag;
use crate::usecase::git_bump::MockGitDataProvider;

const APP_NAME: &str = "app";
const FORMAT: &str = "v{version}";

fn setup_git_provider() -> MockGitDataProvider {
    let mut git_provider = MockGitDataProvider::new();
    git_provider
        .expect_find_latest_version()
        .with(eq(APP_NAME))
        .times(1)
        .returning(|_| Ok(Some(Version::new(1, 2, 3))));
    // git_provider
    //     .expect_release()
    //     .with(eq(APP_NAME), eq(Tag::new_with_format(FORMAT, APP_NAME, Version::new(1, 2, 4))), eq(String::from("## What's Changed\n\n")))
    //     .times(1)
    //     .returning(|_, _, _| Ok(()));

    git_provider
}

#[test]
fn when_app_name_and_component_are_provided_and_there_are_no_commits_then_version_should_be_bumped() {
    let mut git_provider = setup_git_provider();

    git_provider.expect_release().times(0);
    git_provider.expect_get_commits().times(1).returning(|_, _| Ok(vec![]));

    let use_case = AppReleaseUseCase {
        git_provider: Box::new(git_provider),
        format: FORMAT.to_string()
    };

    let request = AppReleaseUseCaseRequest {
        app_name: APP_NAME.to_string(),
        component: Component::Patch
    };

    let result = use_case.execute(request);
    assert!(result.is_err());

    if let Err(error) = result {
        assert!(error == AppReleaseUseCaseError::NoChanges);
    } else {
        panic!("Expected error");
    }
}

// #[test]
// fn when_app_name_and_component_are_provided_and_there_are_commits_then_version_should_be_bumped() {
//     let mut git_provider = setup_git_provider();
//
//     let expected_tag = Tag::new_with_format(FORMAT, APP_NAME, Version::new(0, 2, 0));
//
//     // git_provider.expect_find_latest_version()
//     //     .with(eq("vemo"))
//     //     .returning(|_| Ok(Some(Version::new(0, 1, 0))));
//     //
//     // git_provider.expect_release()
//     //     .with(eq("vemo"), eq(expected_tag.clone()), eq(String::new()))
//     //     .returning(|_, _, _| Ok(()));
//
//     // git_provider.expect_get_commits()
//
//     let use_case = AppReleaseUseCase {
//         git_provider: Box::new(git_provider),
//         format: FORMAT.to_string()
//     };
//
//     let request = AppReleaseUseCaseRequest {
//         app_name: APP_NAME.to_string(),
//         component: Component::Minor,
//     };
//
//     let result = use_case.execute(request);
//
//     assert!(result.is_ok());
//     let response = result.unwrap_or_else(|_| panic!("Expected Ok, but got Err"));
//     assert_eq!(response.tag, expected_tag);
// }
