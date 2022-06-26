use std::fmt::{Display, Formatter};

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
