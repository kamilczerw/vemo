use crate::dataprovider::git::GitDataProvider;
use crate::dataprovider::github::{GithubDataProvider, GithubDataProviderError, HttpClient};
use crate::usecase::release::Commit;

impl GithubDataProvider {
    pub fn new(git_client: GitDataProvider, http_client: Box<dyn HttpClient>) -> GithubDataProvider {
        Self { git_client, http_client }
    }

    pub async fn get_commit_author(&self, commit: &Commit) -> Result<String, GithubDataProviderError> {
        // let url = format!("https://api.github.com/repos/{}/{}/commits/{}", self.owner, self.repo, commit.id);
        // let response = self.http_client.get(&url)?;
        // let json: serde_json::Value = serde_json::from_str(&response)?;
        // let author = json["commit"]["author"]["name"].as_str().unwrap_or("unknown");
        // Ok(author.to_string())
        todo!()
    }
}
