use std::fmt::{Display, Formatter};
use semver::Version;
use crate::commands::Component;

#[derive(Debug, Clone, Eq, Ord, PartialEq)]
pub struct Tag {
    pub format: String,
    pub raw: String,
    pub version: Version,
    pub app_name: String
}

impl Tag {
    pub fn new(format: &str, raw: &str, version: Version, app_name: &str) -> Tag {
        Tag { format: format.to_string(), raw: raw.to_string(), version, app_name: app_name.to_string() }
    }

    pub fn new_with_format(format: &str, app_name: &str, version: Version) -> Tag {
        Tag::new(format, Self::raw_version(format, app_name, &version).as_str(), version, app_name)
    }

    pub fn bump(mut self, component: &Component) -> Self {
        let version = &self.version;
        let new_version = match component {
            Component::Major => Version::new(version.major + 1, 0, 0),
            Component::Minor => Version::new(version.major, version.minor + 1, 0),
            Component::Patch => Version::new(version.major, version.minor, version.patch + 1),
        };

        self.version = new_version;
        self.raw = Self::raw_version(&self.format, &self.app_name, &self.version);

        self
    }

    pub fn formatted(&self) -> String {
        Self::raw_version(&self.format, &self.app_name, &self.version)
    }

    fn raw_version(format: &str, app_name: &str, version: &Version) -> String {
        format
            .replace("{version}", format!("{}", version).as_str())
            .replace("{app_name}", app_name)

    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted())
    }
}

impl PartialOrd for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.version.cmp(&other.version))
    }
}
