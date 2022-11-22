use semver::Version;
use rstest::fixture;
use crate::cfg::DEFAULT_TAG_FORMAT;
use crate::git::model::repo::Repo;
use crate::git::model::tag::Tag;
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
        raw_url: "".to_string(),
        repo_name: "".to_string(),
        provider: Default::default(),
        repo_type: Default::default()
    });

    empty_git_client
}
