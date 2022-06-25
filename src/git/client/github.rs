use reqwest::blocking::Client;
use crate::commands::error::CommandError;
use crate::git::client::error::GitClientError;
use crate::git::GitClient;
use crate::commands::shell::git::{Git, Repo, Tag};

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
    fn create_release(&self, name: String, tag: Tag, body: String) -> Result<(), GitClientError> {
        let body = format!(
            r#"{{
                "tag_name": "{tag}",
                "target_commitish": "master",
                "name": "{name}",
                "body": "{body}",
                "draft": false,
                "prerelease": false
            }}"#,
            tag = tag.
        );
        let res = self.http.post(&format!("{}/repos/{}/releases", self.api, self.repo))
            .header("Authorization", format!("token {}", self.token))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .map_err(|e| GitClientError::RequestError(e))?;
        // let res = &self.http.post(&self.url)
        //     .body("the exact body that is sent")
        //     .send()?;
    }
}

impl From<GitClientError> for CommandError {
    fn from(err: GitClientError) -> Self {
        CommandError::ParseError("Make more granular error handling!!!".to_string())
    }
}
