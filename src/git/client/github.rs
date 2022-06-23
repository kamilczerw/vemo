use reqwest::blocking::Client;
use crate::commands::error::CommandError;
use crate::git::client::error::GitClientError;
use crate::git::GitClient;
use crate::commands::shell::git::{Git, Repo};

pub struct GithubClient {
    token: String,
    http: Client,
    api: String,
    repo: String
}

impl GithubClient {
    /// Create a new GithubClient
    /// # Arguments
    /// * `token` - Github token
    /// * `repo` - Repo object
    pub fn new(token: String, repo: Repo) -> Result<GithubClient, GitClientError> {
        Ok(GithubClient {
            token,
            http: Client::new(),
            api: "https://api.github.com".to_string(),
            repo: repo.repo_name
        })
    }
}

impl GitClient for GithubClient {
    /// Create a new Github release
    fn create_release(&self) -> Result<(), GitClientError> {
        // let res = &self.http.post(&self.url)
        //     .body("the exact body that is sent")
        //     .send()?;
        todo!()
    }
}

impl From<CommandError> for GitClientError {
    fn from(err: CommandError) -> Self {
        GitClientError::UnexpectedError(format!("{:?}", err))
    }
}
