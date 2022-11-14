use mockall::predicate::*;
use semver::Version;

use crate::commands::shell::git::{Git, GitProvider, RepoType, Tag};
use crate::commands::shell::MockGitCli;

static TAG_FORMAT: &str = "{app_name}/v{version}";

static VALID_APPS: &str = "app/v0.1.0\ngateway/v0.0.1\napp/v0.1.1\napp/v1.0.0\ngateway/v1.0.0";

#[test]
fn get_tags_should_extract_tags_sorted_by_version_descending() {
    let mut git_cli_mock = MockGitCli::new();
    git_cli_mock.expect_get_tags()
        .returning(|_| Ok(VALID_APPS.to_string()));

    let git = Git::new(Box::new(git_cli_mock), TAG_FORMAT.to_string());
    let tags = git.get_tags(None).unwrap();

    assert_eq!(tags.len(), 5);
    assert_eq!(tags[0], Tag::new(TAG_FORMAT, "gateway/v1.0.0", Version::parse("1.0.0").unwrap(), "gateway"));
    assert_eq!(tags[1], Tag::new(TAG_FORMAT, "app/v1.0.0", Version::parse("1.0.0").unwrap(), "app"));
    assert_eq!(tags[2], Tag::new(TAG_FORMAT, "app/v0.1.1", Version::parse("0.1.1").unwrap(), "app"));
    assert_eq!(tags[3], Tag::new(TAG_FORMAT, "app/v0.1.0", Version::parse("0.1.0").unwrap(), "app"));
    assert_eq!(tags[4], Tag::new(TAG_FORMAT, "gateway/v0.0.1", Version::parse("0.0.1").unwrap(), "gateway"));
}

#[test]
fn get_latest_tags_should_extract_only_latest_tags_for_all_apps() {
    let mut git_cli_mock = MockGitCli::new();
    git_cli_mock.expect_get_tags()
        .returning(|_| Ok(VALID_APPS.to_string()));
    let git = Git::new(Box::new(git_cli_mock), TAG_FORMAT.to_string());
    let tags = git.get_latest_tags().unwrap();

    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0], Tag::new(TAG_FORMAT, "gateway/v1.0.0", Version::parse("1.0.0").unwrap(), "gateway"));
    assert_eq!(tags[1], Tag::new(TAG_FORMAT, "app/v1.0.0", Version::parse("1.0.0").unwrap(), "app"));
}

#[test]
fn get_latest_tag_for_specific_app_should_return_a_tag() {
    let mut git_cli_mock = MockGitCli::new();
    git_cli_mock.expect_get_tags()
        .returning(|_| Ok(VALID_APPS.to_string()));
    let git = Git::new(Box::new(git_cli_mock), TAG_FORMAT.to_string());
    let tag = git.find_latest_tag("gateway").unwrap();

    assert_eq!(tag, Some(Tag::new(TAG_FORMAT, "gateway/v1.0.0", Version::parse("1.0.0").unwrap(), "gateway")));
}

#[test]
fn get_repo_info_with_valid_github_ssh_url_should_return_a_repo_info() {
    let mut mock = MockGitCli::new();
    mock.expect_get_config()
        .returning(|_| Ok("git@github.com:kamilczerw/vemo.git".to_string()));

    let git = Git::new(Box::new(mock), TAG_FORMAT.to_string());

    let repo_info = git.get_repo_info().unwrap();

    assert_eq!(repo_info.repo_name, "kamilczerw/vemo");
    assert_eq!(repo_info.raw_url, "git@github.com:kamilczerw/vemo.git".to_string());
    assert_eq!(repo_info.repo_type, RepoType::Ssh);
    assert_eq!(repo_info.provider, GitProvider::Github);
}

#[test]
fn get_repo_info_with_valid_github_ssh_url_with_new_line_at_the_end_should_return_a_repo_info() {
    let mut mock = MockGitCli::new();
    mock.expect_get_config()
        .returning(|_| Ok("git@github.com:kamilczerw/vemo.git\n".to_string()));

    let git = Git::new(Box::new(mock), TAG_FORMAT.to_string());

    let repo_info = git.get_repo_info().unwrap();

    assert_eq!(repo_info.repo_name, "kamilczerw/vemo");
    assert_eq!(repo_info.raw_url, "git@github.com:kamilczerw/vemo.git".to_string());
    assert_eq!(repo_info.repo_type, RepoType::Ssh);
    assert_eq!(repo_info.provider, GitProvider::Github);
}

#[test]
fn get_repo_info_with_valid_github_http_url_should_return_a_repo_info() {
    let mut mock = MockGitCli::new();
    mock.expect_get_config()
        .returning(|_| Ok("https://github.com/kamilczerw/vemo.git".to_string()));

    let git = Git::new(Box::new(mock), TAG_FORMAT.to_string());

    let repo_info = git.get_repo_info().unwrap();

    assert_eq!(repo_info.repo_name, "kamilczerw/vemo");
    assert_eq!(repo_info.raw_url, "https://github.com/kamilczerw/vemo.git");
    assert_eq!(repo_info.repo_type, RepoType::Http);
    assert_eq!(repo_info.provider, GitProvider::Github);
}

#[test]
fn get_repo_info_with_valid_github_http_url_but_skipping_protocol_should_return_a_repo_info() {
    let mut mock = MockGitCli::new();
    mock.expect_get_config()
        .returning(|_| Ok("github.com/kamilczerw/vemo.git".to_string()));

    let git = Git::new(Box::new(mock), TAG_FORMAT.to_string());

    let repo_info = git.get_repo_info().unwrap();

    assert_eq!(repo_info.repo_name, "kamilczerw/vemo");
    assert_eq!(repo_info.raw_url, "github.com/kamilczerw/vemo.git");
    assert_eq!(repo_info.repo_type, RepoType::Http);
    assert_eq!(repo_info.provider, GitProvider::Github);
}

#[test]
fn get_repo_info_with_invalid_http_url_should_return_a_failure() {
    let mut mock = MockGitCli::new();
    mock.expect_get_config()
        .returning(|_| Ok("test://github.com/kamilczerw/vemo.git".to_string()));

    let git = Git::new(Box::new(mock), TAG_FORMAT.to_string());

    let repo_info = git.get_repo_info();
    assert!(repo_info.is_err());
}

#[test]
fn get_repo_info_with_invalid_ssh_url_should_return_a_failure() {
    let mut mock = MockGitCli::new();
    mock.expect_get_config()
        .returning(|_| Ok("ssh@github.com:kamilczerw/vemo.git".to_string()));

    let git = Git::new(Box::new(mock), TAG_FORMAT.to_string());

    let repo_info = git.get_repo_info();
    assert!(repo_info.is_err());
}

#[test]
fn get_repo_info_with_invalid_provider_should_return_a_repo_info_with_unknown_provider() {
    let mut mock = MockGitCli::new();
    mock.expect_get_config()
        .returning(|_| Ok("git@invalid.com:kamilczerw/vemo.git".to_string()));

    let git = Git::new(Box::new(mock), TAG_FORMAT.to_string());

    let repo_info = git.get_repo_info().unwrap();

    assert_eq!(repo_info.repo_name, "kamilczerw/vemo");
    assert_eq!(repo_info.raw_url, "git@invalid.com:kamilczerw/vemo.git".to_string());
    assert_eq!(repo_info.repo_type, RepoType::Ssh);
    assert_eq!(repo_info.provider, GitProvider::Unknown);
}
