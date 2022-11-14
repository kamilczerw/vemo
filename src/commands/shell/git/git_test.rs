use crate::commands::shell::MockGitCli;
use crate::commands::shell::git::{GitProvider, RepoType};
use mockall::predicate::*;

use super::Git;

#[test]
fn get_repo_info_should_return_repo_info_for_ssh_repo() {
    let mut git_cli_mock = MockGitCli::new();

    git_cli_mock.expect_get_config()
        .with(eq("remote.origin.url"))
        .returning(|_| Ok(String::from("git@github.com:abc/def.git")));

    let git = Git::new(Box::new(git_cli_mock), String::from(""));

    let info = git.get_repo_info().ok().unwrap();
    assert_eq!(GitProvider::Github, info.provider);
    assert_eq!(RepoType::Ssh, info.repo_type);
    assert_eq!("git@github.com:abc/def.git", info.raw_url);
    assert_eq!("https://github.com/abc/def", info.http_url());
    assert_eq!("abc/def", info.repo_name);
}

#[test]
fn get_repo_info_should_return_repo_info_for_http_repo() {
    let mut git_cli_mock = MockGitCli::new();

    git_cli_mock.expect_get_config()
    .with(eq("remote.origin.url"))
    .returning(|_| Ok(String::from("https://github.com/abc/def.git")));

    let git = Git::new(Box::new(git_cli_mock), String::from(""));

    let info = git.get_repo_info().ok().unwrap();
    assert_eq!(GitProvider::Github, info.provider);
    assert_eq!(RepoType::Http, info.repo_type);
    assert_eq!("https://github.com/abc/def.git", info.raw_url);
    assert_eq!("https://github.com/abc/def", info.http_url());
    assert_eq!("abc/def", info.repo_name);
}

#[test]
fn get_repo_info_should_return_repo_info_for_http_repo_without_https() {
    let mut git_cli_mock = MockGitCli::new();

    git_cli_mock.expect_get_config()
        .with(eq("remote.origin.url"))
        .returning(|_| Ok(String::from("github.com/abc/def.git")));

    let git = Git::new(Box::new(git_cli_mock), String::from(""));

    let info = git.get_repo_info().ok().unwrap();
    assert_eq!(GitProvider::Github, info.provider);
    assert_eq!(RepoType::Http, info.repo_type);
    assert_eq!("github.com/abc/def.git", info.raw_url);
    assert_eq!("https://github.com/abc/def", info.http_url());
    assert_eq!("abc/def", info.repo_name);
}
