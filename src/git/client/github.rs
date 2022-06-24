use reqwest::blocking::Client;
use crate::commands::error::CommandError;
use crate::git::client::error::GitClientError;
use crate::git::GitClient;
use crate::commands::shell::git::{Git, Repo};

pub struct GithubClient {
    pub token: String,
    pub http: Client,
    pub api: String,
    pub repo: String
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
        let res = self.http.post(&format!("{}/repos/{}/releases", self.api, self.repo))
            .header("Authorization", format!("token {}", self.token))
            .header("Content-Type", "application/json")
            .body(r#"{
                "tag_name": "v1.0.0",
                "target_commitish": "master",
                "name": "v1.0.0",
                "body": "Release v1.0.0",
                "draft": false,
                "prerelease": false
            }"#)
            .send()
            .map_err(|e| GitClientError::RequestError(e))?;
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
