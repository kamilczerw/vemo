use semver::Version;
use rstest::fixture;
use crate::cfg::DEFAULT_TAG_FORMAT;
use crate::git::model::repo::{Repo, RepoType};
use crate::git::model::tag::Tag;
use crate::git::provider::Provider;
use super::MockGitClient;

pub fn tag(version: &str, app_name: &str) -> Tag {
    Tag {
        format: DEFAULT_TAG_FORMAT.to_string(),
        raw: format!("{}-{}", app_name, version),
        version: Version::parse(version).unwrap(),
        app_name: app_name.to_string()
    }
}

#[fixture]
pub(crate) fn empty_git_client() -> MockGitClient {
    MockGitClient::new()
}

#[fixture]
pub(crate) fn git_client(mut empty_git_client: MockGitClient) -> MockGitClient {
    empty_git_client.expect_get_repo().returning(|| Repo {
        raw_url: "git@github.com:abc/def.git".to_string(),
        repo_name: "abc/def".to_string(),
        provider: Provider::Github,
        repo_type: RepoType::Ssh
    });

    empty_git_client
}
