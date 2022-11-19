use crate::git;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Default)]
pub struct Repo {
    pub raw_url: String,
    pub repo_name: String,
    pub provider: git::Provider,
    pub repo_type: RepoType
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Default)]
pub enum RepoType {
    #[default]
    Ssh,
    Http
}

impl Repo {
    pub fn http_url(&self) -> String {
        match self.repo_type {
            RepoType::Ssh => ssh_to_http_url(&self.raw_url),
            RepoType::Http => parse_http_raw_url(&self.raw_url)
        }
    }
}

// Coverts git@github.com:abc/def.git to https://github.com/abc/def
fn ssh_to_http_url(ssh: &str) -> String {
    let mut parts = ssh.split(':');
    let host = parts.next().unwrap().replace("git@", "");
    let repo = parts.next().unwrap().replace(".git", "");
    format!("https://{}/{}", host, repo)
}

// Converts:
//  - https://github.com/abc/def.git to https://github.com/abc/def
//  - github.com/abc/def.git to https://github.com/abc/def
fn parse_http_raw_url(raw_url: &str) -> String {
    raw_url.starts_with("https://")
        .then(|| raw_url.replace(".git", ""))
        .unwrap_or_else(|| format!("https://{}", raw_url.replace(".git", "")))
}
