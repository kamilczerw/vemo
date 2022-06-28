use std::collections::HashMap;
use reqwest::blocking::Client;
use crate::commands::error::CommandError;
use crate::git::client::error::GitClientError;
use crate::git::GitClient;
use crate::git::model::{Change, Repo};
use serde_json::json;
use crate::cfg::AppConfig;
use crate::Git;
use crate::git::model::Release;

pub struct GithubClient {
    pub token: String,
    pub http: Client,
    pub api: String,
    pub repo: String,
    git: Git,
    app_configs: HashMap<String, AppConfig>,
}

impl GithubClient {
    /// Create a new GithubClient
    /// # Arguments
    /// * `token` - Github token
    /// * `repo` - Repo object
    pub fn new(token: String, repo: Repo, git_cli: Git, app_configs: HashMap<String, AppConfig>) -> Result<GithubClient, GitClientError> {
        Ok(GithubClient {
            token,
            http: Client::new(),
            api: "https://api.github.com".to_string(),
            repo: repo.repo_name,
            git: git_cli,
            app_configs
        })
    }
}

impl GitClient for GithubClient {
    /// Create a new Github release
    fn create_release(&self, name: String, tag: Release, description: String) -> Result<(), GitClientError> {
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

    fn latest_release(&self, name: &str) -> Result<Option<Release>, GitClientError> {
        self.git.find_latest_tag(name)
            .map_err(|e| GitClientError::GitCliError(e))
    }

    fn get_changelog(&self, tag: Option<Release>, app_name: &str) -> Result<Vec<Change>, GitClientError> {
        let dir = self.app_configs.get(app_name)
            .map(|c| c.path.clone())
            .flatten()
            .ok_or(GitClientError::MissingAppConfig(app_name.to_string()))?;
        
        self.git.get_commits(tag, &dir)
            .map_err(|e| GitClientError::GitCliError(e))
    }

    fn list_latest_releases(&self) -> Result<Vec<Release>, GitClientError> {
        todo!()
    }
}

impl From<GitClientError> for CommandError {
    fn from(err: GitClientError) -> Self {
        CommandError::GitClientError(err)
    }
}
