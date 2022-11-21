use mockall::automock;
use semver::Version;
use crate::git::model::tag::Tag;
use crate::usecase::UseCase;

pub struct AppReleaseUseCase {
    pub(crate) git_provider: Box<dyn GitDataProvider>,
    pub(crate) config_data_provider: Box<dyn ConfigDataProvider>,
    pub(crate) format: String,
}


impl UseCase<AppReleaseUseCaseRequest, AppReleaseUseCaseResponse, AppReleaseUseCaseError> for AppReleaseUseCase {
    fn execute(&self, params: AppReleaseUseCaseRequest) -> Result<AppReleaseUseCaseResponse, AppReleaseUseCaseError> {
        let latest_version = self.git_provider.find_latest_version(params.app_name.as_str())?;

        let version = match latest_version {
            Some(version) => version,
            None => Version::new(0, 1, 0)
        };
        let tag = Tag::new_with_format(self.format.as_str(), params.app_name.as_str(), version);
        let new_tag = tag.clone().bump_v2(&params.component);

        let commits = self.git_provider.get_commits(&tag, "path")?;

        if commits.is_empty() {
            return Err(AppReleaseUseCaseError::NoChanges)
        }

        let mut body = String::from("## What's Changed\n\n");
        for commit in commits {
            body.push_str(&format!("* {} by {}\n", commit.message, commit.author));
        }

        if let Some(url) = self.git_provider.compare_url(&tag, &new_tag) {
            body.push_str(&format!("\n\n**Full Changelog**: {}", url));
        }

        self.git_provider.release(params.app_name.as_str(), &new_tag, &body)?;

        Ok(AppReleaseUseCaseResponse {
            tag: new_tag,
            body
        })
    }
}

impl AppReleaseUseCase {
    pub fn release_body(&self, tag: &Tag) -> String {
        // self.git_provider.get_commits(tag, path)
        todo!("Implement release body")
    }
}

pub struct AppReleaseUseCaseRequest {
    pub(crate) app_name: String,
    pub(crate) component: Component
}

pub struct AppReleaseUseCaseResponse {
    pub(crate) tag: Tag,
    pub(crate) body: String
}

#[derive(Eq, PartialEq, Debug)]
pub enum AppReleaseUseCaseError {
    UnexpectedError,
    NoChanges
}

pub enum Component {
    Major,
    Minor,
    Patch
}

pub struct Commit {
    pub(crate) hash: String,
    pub(crate) message: String,
    pub(crate) author: String, // TODO: change to author object
}

#[automock]
pub trait GitDataProvider {
    fn find_latest_version(&self, app_name: &str) -> Result<Option<Version>, GitDataProviderError>;
    fn release(&self, name: &str, tag: &Tag, body: &String) -> Result<(), GitDataProviderError>;
    fn get_commits(&self, tag: &Tag, path: &str) -> Result<Vec<Commit>, GitDataProviderError>;
    fn compare_url(&self, tag: &Tag, new_tag: &Tag) -> Option<String>;
}

#[automock]
pub trait ConfigDataProvider {

}

pub enum GitDataProviderError {
    UnexpectedError
}

impl From<GitDataProviderError> for AppReleaseUseCaseError {
    fn from(error: GitDataProviderError) -> Self {
        match error {
            GitDataProviderError::UnexpectedError => AppReleaseUseCaseError::UnexpectedError
        }
    }
}
