use std::thread::sleep;
use async_std::task;
use futures::future::join_all;
use futures::{future, StreamExt};
use crate::dataprovider::github::GithubDataProvider;
use crate::git::model::tag::Tag;
use crate::usecase::release::{Commit, GitDataProviderError, ReleaseDataProvider};

impl ReleaseDataProvider for GithubDataProvider {
    fn find_latest_tag(&self, app_name: &str) -> Result<Option<Tag>, GitDataProviderError> {
        self.git_client.find_latest_tag(app_name)
    }

    fn release(&self, _name: &str, _tag: &Tag, _body: &String) -> Result<(), GitDataProviderError> {
        todo!("Implement GithubDataProvider::find_latest_version")
    }

    fn get_commits(&self, tag: &Option<Tag>, path: Option<String>) -> Result<Vec<Commit>, GitDataProviderError> {
        task::block_on(self.get_commits_async(tag, path))
    }

    fn compare_url(&self, _tag: &Option<Tag>, _new_tag: &Tag) -> Result<Option<String>, GitDataProviderError> {
        todo!("Implement GithubDataProvider::find_latest_version")
    }
}

use rand::Rng;

impl GithubDataProvider {
    async fn get_commits_async(&self, tag: &Option<Tag>, path: Option<String>) -> Result<Vec<Commit>, GitDataProviderError> {
        let commits = self.git_client.get_commits(tag, path)?;
        let commits = commits.into_iter()
            .map(move |commit| async move {
                if let Ok(author) = self.get_commit_author(&commit).await {
                    Commit { hash: commit.hash, message: commit.message, author }
                } else {
                    log::warn!("Failed to get author for commit {}", &commit.hash);
                    commit
                }
            })
            // .map(task::spawn) // TODO: run in parallel
            .collect::<Vec<_>>();

        Ok(future::join_all(commits).await)
    }
}
