use semver::Version;
use rstest::fixture;
use crate::cfg::DEFAULT_TAG_FORMAT;
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
pub(crate) fn git_client() -> MockGitClient {
    MockGitClient::new()
}
