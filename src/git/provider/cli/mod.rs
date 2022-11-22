use regex::Regex;

use error::CliProviderError;

use crate::git;
use crate::git::model::repo::{Repo, RepoType};
use crate::git::shell::git_cli::ShellGit;

mod provider_test;
pub(crate) mod error;

#[derive(Default)]
pub struct Provider {
    pub repo: Repo
}

impl Provider {
    pub fn init() -> Result<Provider, CliProviderError> {
        let repo = Self::get_repo()?;

        Ok(Provider { repo })
    }

    fn get_repo() -> Result<Repo, CliProviderError> {
        let repo_url = ShellGit::get_config("remote.origin.url")?;
        let repo_url = repo_url.as_str();
        let repo_url = repo_url.strip_suffix("\n").unwrap_or(repo_url);

        let ssh_re = Regex::new(r"^git@(?P<provider>[a-zA-Z0-9._-]+):(?P<repo>.*)\.git$").unwrap();
        let http_re = Regex::new(r"^(https?://)?(?P<provider>[a-zA-Z0-9._-]+)/(?P<repo>.*)\.git$").unwrap();

        let (repo_type, caps) = if ssh_re.is_match(repo_url) {
            (RepoType::Ssh, ssh_re.captures(repo_url).unwrap())
        } else if http_re.is_match(repo_url) {
            (RepoType::Http, http_re.captures(repo_url).unwrap())
        } else {
            return Err(CliProviderError::UnexpectedError(format!("Invalid repo url {}", repo_url).to_string()))
        };

        let provider = match caps.name("provider") {
            Some(provider) => {
                match provider.as_str() {
                    "github.com" => git::Provider::Github,
                    _ => git::Provider::Unknown
                }
            }
            None => git::Provider::Unknown
        };

        let repo_name = match caps.name("repo") {
            Some(repo) => repo.as_str().to_string(),
            None => return Err(CliProviderError::UnexpectedError("Failed to parse repo name".to_string()))
        };

        Ok(Repo {
            raw_url: repo_url.to_string(),
            provider,
            repo_name,
            repo_type
        })
    }
}
