use std::collections::HashMap;
use crate::commands::error::CommandError;
use crate::git::client::error::GitClientError;
use crate::git::GitClient;
use crate::git::model::{Change, Repo};
use serde_json::{json, Value};
use crate::cfg::AppConfig;
use crate::Git;
use crate::git::model::Release;
use crate::lib::http::{Client, HttpClient, HttpClientError};

/// Github client
/// This client is used to interact with the repository using Github API.
pub struct GithubClient {
    pub http: Box<dyn Client>,
    pub repo: String,
    git: Git,
    app_configs: HashMap<String, AppConfig>,
}

impl GithubClient {
    /// Create a new GithubClient
    pub fn new(token: String, repo: Repo, git_cli: Git, app_configs: HashMap<String, AppConfig>) -> Result<GithubClient, GitClientError> {
        Ok(GithubClient {
            http: Box::new(HttpClient::new("https://api.github.com", Some(token))),
            repo: repo.repo_name,
            git: git_cli,
            app_configs
        })
    }

    pub fn init(http: Box<dyn Client>, repo: String, git: Git, app_configs: HashMap<String, AppConfig>) -> Result<GithubClient, GitClientError> {
        Ok(GithubClient {
            http,
            repo,
            git,
            app_configs
        })
    }

//     fn _http_get(&self, path: &str, params: HashMap<&str, &str>) -> Result<Response, GitClientError> {
//         let url = format!("{}/{}", self.api, path);
//         let builder = self.http.get(&url).query(&params);
//         let res = self.http_request(builder)?;
//
//         Ok(res)
//     }
//
//     fn http_post(&self, path: &str, body: Value) -> Result<Response, GitClientError> {
//         let url = format!("{}/{}", self.api, path.trim_start_matches("/"));
//         let builder = self.http.post(&url)
//             .body(serde_json::to_string(&body).unwrap());
//         let res = self.http_request(builder)?;
//
//         Ok(res)
//     }
//
//     fn http_request(&self, req: RequestBuilder) -> Result<Response, GitClientError> {
//         let res = req
//             .header("Authorization", format!("token {}", self.token))
//             .header("User-Agent", "Vemo-Cli")
//             .header("Content-Type", "application/json")
//             .send()
//             .map_err(|e| GitClientError::RequestError(e))?;
//
//         Ok(res)
//     }
}

impl GitClient for GithubClient {

    /// Create a new Github release
    fn create_release(&self, name: String, tag: Release, description: String) -> Result<(), GitClientError> {
        let body = json!({
            "tag_name": tag.raw,
            "name": name,
            "body": description,
        });

        self.http.post(&format!("/repos/{}/releases", self.repo), body)?;

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

impl From<HttpClientError> for GitClientError {
    fn from(err: HttpClientError) -> Self {
        match err {
            HttpClientError::RequestError(err) => GitClientError::RequestError(err),
        }
    }
}
