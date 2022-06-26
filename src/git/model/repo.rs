use crate::commands::shell::git::GitProvider;

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
