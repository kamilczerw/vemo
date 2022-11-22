use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum GitProvider {
    Github,
}

impl GitProvider {
    pub fn env_name(&self) -> String {
        match self {
            GitProvider::Github => "VEMO_GITHUB_TOKEN".to_string(),
        }
    }

    pub fn setting_name(&self) -> String {
        match self {
            GitProvider::Github => "github.token".to_string(),
        }
    }
}

impl Display for GitProvider {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            GitProvider::Github => write!(f, "github"),
        }
    }
}
