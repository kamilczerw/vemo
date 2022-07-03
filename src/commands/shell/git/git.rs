use std::collections::HashMap;
use log::debug;
use regex::{Captures, Regex};
use semver::Version;
use crate::commands::shell::git::GitCliError;
use crate::git::model::GitProvider;
use crate::commands::shell::git_cli::ShellGit;
use crate::commands::shell::GitCli;
use crate::git::model::Change;
use crate::git::model::{Repo, RepoType};
use crate::git::model::Release;
use mockall::*;

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
    pub fn get_tags(&self, app_name_filter: Option<String>) -> Result<Vec<Release>, GitCliError> {
        let format = self.tag_format.clone();
        let filter = format
            .replace("{version}", "*");
        let app_name = match app_name_filter {
            None => String::from("*"),
            Some(name) => name
        };
        let filter = filter.replace("{app_name}", &app_name);
        debug!("git tags filter: {}", filter);
        let raw_output = self.git.get_tags(filter)?;
        let mut tags = Self::parse_tags(raw_output.clone(), format);
        tags.sort();
        tags.reverse();

        Ok(tags)
    }

    /// List latest versions for each application
    pub fn get_latest_tags(&self) -> Result<Vec<Release>, GitCliError> {
        let mut tags: HashMap<String, Release> = HashMap::new();
        let mut result: Vec<Release> = vec![];
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

    pub fn find_latest_tag(&self, app_name: &str) -> Result<Option<Release>, GitCliError> {
        let tags = self.get_tags(Some(app_name.to_string()))?;
        debug!("Found {} tags for app {}, tags: {:?}", &tags.len(), app_name, &tags);
        let tag = tags.first();

        Ok(tag.map(|t| t.clone()))
    }

    pub fn get_config(&self, key: &str) -> Result<String, GitCliError> {
        self.git.get_config(key)
    }

    pub fn get_repo_info(&self) -> Result<Repo, GitCliError> {
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
            return Err(GitCliError::ParseError(format!("Invalid repo url {}", repo_url).to_string()))
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
            None => return Err(GitCliError::ParseError("Failed to parse repo name".to_string()))
        };

        Ok(Repo {
            git_url: repo_url.to_string(),
            provider,
            repo_name,
            repo_type
        })
    }

    pub fn get_commits(&self, tag: Option<Release>, dir: &str) -> Result<Vec<Change>, GitCliError> {
        let tag = tag.map(|t| t.formatted());
        self.git.get_commits(tag, dir)
    }

    fn parse_tags(raw_tags: String, format: String) -> Vec<Release> {
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
                    tags.push(Release::new(format.clone().as_str(), &raw, version, &app_name));
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
