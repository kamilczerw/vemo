use semver::Version;
use crate::commands::error::CommandError;
use crate::commands::shell::git::{Git, GitProvider, RepoType, Tag};
use crate::commands::shell::GitCli;
use mockall::*;
use mockall::predicate::*;

struct  ValidGitCli {}
impl GitCli for ValidGitCli {
    fn get_tags(&self, _filter: String) -> Result<String, CommandError> {
        Ok(format!(
            "app/v0.1.0\n\
            gateway/v0.0.1\n\
            app/v0.1.1\n\
            app/v1.0.0\n\
            gateway/v1.0.0"
        ))
    }

    fn get_config(&self, _key: &str) -> Result<String, CommandError> {
        Ok("git@github.com:kamilczerw/vemo.git".to_string())
    }
}

#[test]
fn get_tags_should_extract_tags_sorted_by_version_descending() {
    let git = Git::new(Box::new(ValidGitCli {}), "{app_name}/v{version}".to_string());
    let tags = git.get_tags(None).unwrap();

    assert_eq!(tags.len(), 5);
    assert_eq!(tags[0], Tag::new("gateway/v1.0.0", Version::parse("1.0.0").unwrap(), "gateway"));
    assert_eq!(tags[1], Tag::new("gateway/v0.0.1", Version::parse("0.0.1").unwrap(), "gateway"));
    assert_eq!(tags[2], Tag::new("app/v1.0.0", Version::parse("1.0.0").unwrap(), "app"));
    assert_eq!(tags[3], Tag::new("app/v0.1.1", Version::parse("0.1.1").unwrap(), "app"));
    assert_eq!(tags[4], Tag::new("app/v0.1.0", Version::parse("0.1.0").unwrap(), "app"));
}

#[test]
fn get_latest_tags_should_extract_only_latest_tags_for_all_apps() {
    let git = Git::new(Box::new(ValidGitCli {}), "{app_name}/v{version}".to_string());
    let tags = git.get_latest_tags().unwrap();

    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0], Tag::new("gateway/v1.0.0", Version::parse("1.0.0").unwrap(), "gateway"));
    assert_eq!(tags[1], Tag::new("app/v1.0.0", Version::parse("1.0.0").unwrap(), "app"));
}

#[test]
fn get_latest_tag_for_specific_app_should_return_a_tag() {
    let git = Git::new(Box::new(ValidGitCli {}), "{app_name}/v{version}".to_string());
    let tag = git.find_latest_tag("gateway").unwrap();

    assert_eq!(tag, Some(Tag::new("gateway/v1.0.0", Version::parse("1.0.0").unwrap(), "gateway")));
}

mock! {
    GC {}

    impl GitCli for GC {
        fn get_config(&self, _key: &str) -> Result<String, CommandError>;
        fn get_tags(&self, _filter: String) -> Result<String, CommandError>;
    }
}

#[test]
fn get_repo_info_with_valid_github_ssh_url_should_return_a_repo_info() {
    let mut mock = MockGC::new();
    mock.expect_get_config()
        .returning(|_| Ok("git@github.com:kamilczerw/vemo.git".to_string()));

    let git = Git::new(Box::new(mock), "{app_name}/v{version}".to_string());

    let repo_info = git.get_repo_info().unwrap();

    assert_eq!(repo_info.repo_name, "kamilczerw/vemo");
    assert_eq!(repo_info.git_url, "git@github.com:kamilczerw/vemo.git".to_string());
    assert_eq!(repo_info.repo_type, RepoType::Ssh);
    assert_eq!(repo_info.provider, GitProvider::Github);
}

#[test]
fn get_repo_info_with_valid_github_ssh_url_with_new_line_at_the_end_should_return_a_repo_info() {
    let mut mock = MockGC::new();
    mock.expect_get_config()
        .returning(|_| Ok("git@github.com:kamilczerw/vemo.git\n".to_string()));

    let git = Git::new(Box::new(mock), "{app_name}/v{version}".to_string());

    let repo_info = git.get_repo_info().unwrap();

    assert_eq!(repo_info.repo_name, "kamilczerw/vemo");
    assert_eq!(repo_info.git_url, "git@github.com:kamilczerw/vemo.git".to_string());
    assert_eq!(repo_info.repo_type, RepoType::Ssh);
    assert_eq!(repo_info.provider, GitProvider::Github);
}

#[test]
fn get_repo_info_with_valid_github_http_url_should_return_a_repo_info() {
    let mut mock = MockGC::new();
    mock.expect_get_config()
        .returning(|_| Ok("https://github.com/kamilczerw/vemo.git".to_string()));

    let git = Git::new(Box::new(mock), "{app_name}/v{version}".to_string());

    let repo_info = git.get_repo_info().unwrap();

    assert_eq!(repo_info.repo_name, "kamilczerw/vemo");
    assert_eq!(repo_info.git_url, "https://github.com/kamilczerw/vemo.git");
    assert_eq!(repo_info.repo_type, RepoType::Http);
    assert_eq!(repo_info.provider, GitProvider::Github);
}

#[test]
fn get_repo_info_with_valid_github_http_url_but_skipping_protocol_should_return_a_repo_info() {
    let mut mock = MockGC::new();
    mock.expect_get_config()
        .returning(|_| Ok("github.com/kamilczerw/vemo.git".to_string()));

    let git = Git::new(Box::new(mock), "{app_name}/v{version}".to_string());

    let repo_info = git.get_repo_info().unwrap();

    assert_eq!(repo_info.repo_name, "kamilczerw/vemo");
    assert_eq!(repo_info.git_url, "github.com/kamilczerw/vemo.git");
    assert_eq!(repo_info.repo_type, RepoType::Http);
    assert_eq!(repo_info.provider, GitProvider::Github);
}

#[test]
fn get_repo_info_with_invalid_http_url_should_return_a_failure() {
    let mut mock = MockGC::new();
    mock.expect_get_config()
        .returning(|_| Ok("test://github.com/kamilczerw/vemo.git".to_string()));

    let git = Git::new(Box::new(mock), "{app_name}/v{version}".to_string());

    let repo_info = git.get_repo_info();
    assert!(repo_info.is_err());
}

#[test]
fn get_repo_info_with_invalid_ssh_url_should_return_a_failure() {
    let mut mock = MockGC::new();
    mock.expect_get_config()
        .returning(|_| Ok("ssh@github.com:kamilczerw/vemo.git".to_string()));

    let git = Git::new(Box::new(mock), "{app_name}/v{version}".to_string());

    let repo_info = git.get_repo_info();
    assert!(repo_info.is_err());
}

#[test]
fn get_repo_info_with_invalid_provider_should_return_a_repo_info_with_unknown_provider() {
    let mut mock = MockGC::new();
    mock.expect_get_config()
        .returning(|_| Ok("git@invalid.com:kamilczerw/vemo.git".to_string()));

    let git = Git::new(Box::new(mock), "{app_name}/v{version}".to_string());

    let repo_info = git.get_repo_info().unwrap();

    assert_eq!(repo_info.repo_name, "kamilczerw/vemo");
    assert_eq!(repo_info.git_url, "git@invalid.com:kamilczerw/vemo.git".to_string());
    assert_eq!(repo_info.repo_type, RepoType::Ssh);
    assert_eq!(repo_info.provider, GitProvider::Unknown);
}
