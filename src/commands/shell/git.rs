use std::collections::HashMap;
use regex::{Captures, Regex};
use semver::Version;
use crate::commands::error::CommandError;
use crate::commands::shell::git_cli::ShellGit;
use crate::commands::shell::GitCli;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Tag {
    pub raw: String,
    pub version: Version,
    pub app_name: String
}

impl Tag {
    pub fn new(raw: &str, version: Version, app_name: &str) -> Tag {
        Tag { raw: raw.to_string(), version, app_name: app_name.to_string() }
    }
}

pub struct Git {
    git: Box<dyn GitCli>,
    tag_format: String
}

impl Git {
    pub fn init(tag_format: String) -> Git {
        Self::new(Box::new(ShellGit {}), tag_format)
    }

    pub fn new(git: Box<dyn GitCli>, tag_format: String) -> Git {
        Git { git, tag_format }
    }

    /// List git tags ordered by version descending
    pub fn get_tags(&self) -> Result<Vec<Tag>, CommandError> {
        let format = self.tag_format.clone();
        let filter = format
            .replace("{version}", "*")
            .replace("{app_name}", "*");
        let raw_output = self.git.get_tags(filter)?;
        let mut tags = Self::parse_tags(raw_output.clone(), format);
        tags.sort();
        tags.reverse();

        Ok(tags)
    }

    pub fn get_latest_tags(&self) -> Result<Vec<Tag>, CommandError> {
        let mut tags: HashMap<String, Tag> = HashMap::new();
        let mut result: Vec<Tag> = vec![];
        for tag_ref in self.get_tags()?.iter() {
            let tag = tag_ref.clone();
            match tags.get(tag.app_name.as_str()) {
                None => {
                    tags.insert(tag.app_name.clone(), tag.clone());
                    result.push(tag);
                }
                Some(_) => {}
            }
        }

        Ok(result)
    }

    fn parse_tags(raw_tags: String, format: String) -> Vec<Tag> {
        let format = format
            .replace("{version}", "(?P<version>[0-9]+\\.[0-9]+\\.[0-9]+)")
            .replace("{app_name}", "(?P<app_name>[0-9a-zA-Z-_]+)");
        let format = format!("(?P<raw>{})\\n{{0,1}}", format);

        let mut tags = vec![];

        let re = Regex::new(format.as_str()).unwrap();
        for caps in re.captures_iter(raw_tags.as_str()) {
            let raw = caps.name("raw").map(|m| String::from(m.as_str()));
            let app_name = caps.name("app_name")
                .map(|m| String::from(m.as_str()));
            let version = Self::get_version(&caps);

            match (raw, app_name, version) {
                (Some(raw), Some(app_name), Some(version)) => {
                    tags.push(Tag::new(&raw, version, &app_name));
                },
                _ => {}
            }
        }

        tags
    }

    fn get_version(caps: &Captures) -> Option<Version> {
        let version = caps.name("version")
            .map(|m| Version::parse(m.as_str()));
        match version {
            Some(Ok(v)) => Some(v),
            _ => None
        }
    }
}
