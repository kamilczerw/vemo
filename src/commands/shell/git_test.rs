use semver::Version;
use crate::commands::error::CommandError;
use crate::commands::shell::git::{Git, Tag};
use crate::commands::shell::GitCli;

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
