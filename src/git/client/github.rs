use reqwest::blocking::Client;
use crate::commands::error::CommandError;
use crate::git::client::error::GitClientError;
use crate::git::GitClient;
use crate::git::model::{Change, Repo};
use serde_json::json;
use crate::git::model::Tag;

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
    fn create_release(&self, name: String, tag: Tag, description: String) -> Result<(), GitClientError> {
        let body = json!({
            "tag_name": tag.raw,
            "name": name,
            "body": description,
        });

        self.http.post(&format!("{}/repos/{}/releases", self.api, self.repo))
            .header("Authorization", format!("token {}", self.token))
            .header("User-Agent", "Vemo-Cli")
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&body).unwrap())
            .send()
            .map_err(|e| GitClientError::RequestError(e))?;

        Ok(())
    }

    fn latest_release(&self, name: &str) -> Result<Option<Tag>, GitClientError> {
        // TODO: use GitCli to get the latest tag
        todo!()
    }

    fn get_changelog(&self, tag: Option<Tag>, app_name: &str) -> Result<Vec<Change>, GitClientError> {
        todo!()
    }
}

impl From<GitClientError> for CommandError {
    fn from(err: GitClientError) -> Self {
        CommandError::GitClientError(err)
    }
}
