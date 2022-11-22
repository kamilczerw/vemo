use crate::dataprovider::git::GitDataProvider;
use crate::dataprovider::github::{GithubDataProvider, GithubDataProviderError, HttpClient};
use crate::usecase::release::Commit;

impl GithubDataProvider {
    pub fn new(git_client: GitDataProvider, http_client: Box<dyn HttpClient>) -> GithubDataProvider {
        Self { git_client, http_client, github_api_url: "https://api.github.com".to_string() }
    }

    pub async fn get_commit_author(&self, commit: &Commit) -> Result<String, GithubDataProviderError> {
        let url = format!("{}/repos/{}/commits/{}", self.github_api_url, self.git_client.get_repo().repo_name, commit.hash);
        let response = self.http_client.get(&url).await?;

        let json: serde_json::Value = serde_json::from_str(&response)?;
        let author = json["author"]["login"].as_str().unwrap_or("unknown");

        Ok(author.to_string())
    }
}

impl From<serde_json::Error> for GithubDataProviderError {
    fn from(error: serde_json::Error) -> Self {
        GithubDataProviderError::UnexpectedError(error.to_string())
    }
}
