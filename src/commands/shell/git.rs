use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use regex::{Captures, Regex};
use semver::Version;
use crate::commands::Component;
use crate::commands::error::CommandError;
use crate::commands::shell::git_cli::ShellGit;
use crate::commands::shell::GitCli;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Tag {
    pub format: String,
    pub raw: String,
    pub version: Version,
    pub app_name: String
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Repo {
    pub git_url: String,
    pub repo_name: String,
    pub provider: GitProvider,
    pub repo_type: RepoType
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum RepoType {
    Ssh,
    Http
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum GitProvider {
    Github,
    Gitlab,
    Bitbucket,
    Unknown
}

impl GitProvider {
    pub fn env_name(&self) -> String {
        match self {
            GitProvider::Github => "VEMO_GITHUB_TOKEN".to_string(),
            GitProvider::Gitlab => "VEMO_GITLAB_TOKEN".to_string(),
            GitProvider::Bitbucket => "VEMO_BITBUCKET_TOKEN".to_string(),
            GitProvider::Unknown => "".to_string()
        }
    }

    pub fn setting_name(&self) -> String {
        match self {
            GitProvider::Github => "github.token".to_string(),
            GitProvider::Gitlab => "gitlab.token".to_string(),
            GitProvider::Bitbucket => "bitbucket.token".to_string(),
            GitProvider::Unknown => "".to_string()
        }
    }
}

impl Display for GitProvider {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            GitProvider::Github => write!(f, "github"),
            GitProvider::Gitlab => write!(f, "gitlab"),
            GitProvider::Bitbucket => write!(f, "bitbucket"),
            GitProvider::Unknown => write!(f, "unknown")
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted())
    }
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
    pub fn get_tags(&self, app_name_filter: Option<String>) -> Result<Vec<Tag>, CommandError> {
        let format = self.tag_format.clone();
        let filter = format
            .replace("{version}", "*");
        let app_name = match app_name_filter {
            None => String::from("*"),
            Some(name) => name
        };
        let filter = filter.replace("{app_name}", &app_name);
        let raw_output = self.git.get_tags(filter)?;
        let mut tags = Self::parse_tags(raw_output.clone(), format);
        tags.sort();
        tags.reverse();

        Ok(tags)
    }

    /// List latest versions for each application
    pub fn get_latest_tags(&self) -> Result<Vec<Tag>, CommandError> {
        let mut tags: HashMap<String, Tag> = HashMap::new();
        let mut result: Vec<Tag> = vec![];
        for tag_ref in self.get_tags(None)?.iter() {
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

    pub fn find_latest_tag(&self, app_name: &str) -> Result<Option<Tag>, CommandError> {
        let tags = self.get_tags(Some(app_name.to_string()))?;
        let tag = tags.first();

        Ok(tag.map(|t| t.clone()))
    }

    pub fn get_config(&self, key: &str) -> Result<String, CommandError> {
        self.git.get_config(key)
    }

    pub fn get_repo_info(&self) -> Result<Repo, CommandError> {
        let repo_url = self.get_config("remote.origin.url")?;
        let repo_url = repo_url.as_str();
        let repo_url = repo_url.strip_suffix("\n").unwrap_or(repo_url);

        let ssh_re = Regex::new(r"^git@(?P<provider>[a-zA-Z0-9._-]+):(?P<repo>.*)\.git$").unwrap();
        let http_re = Regex::new(r"^(https?://)?(?P<provider>[a-zA-Z0-9._-]+)/(?P<repo>.*)\.git$").unwrap();

        let (repo_type, caps) = if ssh_re.is_match(repo_url) {
            (RepoType::Ssh, ssh_re.captures(repo_url).unwrap())
        } else if http_re.is_match(repo_url) {
            (RepoType::Http, http_re.captures(repo_url).unwrap())
        } else {
            return Err(CommandError::ParseError(format!("Invalid repo url {}", repo_url).to_string()))
        };

        let provider = match caps.name("provider") {
            Some(provider) => {
                match provider.as_str() {
                    "github.com" => GitProvider::Github,
                    "gitlab.com" => GitProvider::Gitlab,
                    "bitbucket.com" => GitProvider::Bitbucket,
                    _ => GitProvider::Unknown
                }
            }
            None => GitProvider::Unknown
        };

        let repo_name = match caps.name("repo") {
            Some(repo) => repo.as_str().to_string(),
            None => return Err(CommandError::ParseError("Failed to parse repo name".to_string()))
        };

        Ok(Repo {
            git_url: repo_url.to_string(),
            provider,
            repo_name,
            repo_type
        })
    }

    fn parse_tags(raw_tags: String, format: String) -> Vec<Tag> {
        let regex = format
            .replace("{version}", "(?P<version>[0-9]+\\.[0-9]+\\.[0-9]+)")
            .replace("{app_name}", "(?P<app_name>[0-9a-zA-Z-_]+)");
        let regex = format!("(?P<raw>{})\\n{{0,1}}", regex);

        let mut tags = vec![];

        let re = Regex::new(regex.as_str()).unwrap();
        for caps in re.captures_iter(raw_tags.as_str()) {
            let raw = caps.name("raw").map(|m| String::from(m.as_str()));
            let app_name = caps.name("app_name")
                .map(|m| String::from(m.as_str()));
            let version = Self::get_version(&caps);

            match (raw, app_name, version) {
                (Some(raw), Some(app_name), Some(version)) => {
                    tags.push(Tag::new(format.clone().as_str(), &raw, version, &app_name));
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
