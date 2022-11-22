use semver::Version;
use crate::git::model::tag::Tag;
use crate::usecase::release::{AppReleaseUseCase, AppReleaseUseCaseError, AppReleaseUseCaseRequest, AppReleaseUseCaseResponse};
use crate::usecase::UseCase;

impl UseCase<AppReleaseUseCaseRequest, AppReleaseUseCaseResponse, AppReleaseUseCaseError> for AppReleaseUseCase {
    fn execute(&self, params: AppReleaseUseCaseRequest) -> Result<AppReleaseUseCaseResponse, AppReleaseUseCaseError> {
        let latest_version = self.git_provider.find_latest_version(params.app_name.as_str())?;

        let version = match latest_version {
            Some(version) => version,
            None => Version::new(0, 1, 0)
        };
        let tag = Tag::new_with_format(self.format.as_str(), params.app_name.as_str(), version);
        let new_tag = tag.clone().bump_v2(&params.component);

        let app_config = self.config_data_provider
            .get_app_config(params.app_name.as_str())?;

        let commits = self.git_provider.get_commits(&tag, app_config.path)?;

        if commits.is_empty() {
            return Err(AppReleaseUseCaseError::NoChanges)
        }

        let mut body = String::from("## What's Changed\n\n");
        for commit in commits {
            body.push_str(&format!("* {} by {}\n", commit.message, commit.author));
        }

        if let Ok(Some(url)) = self.git_provider.compare_url(&tag, &new_tag) {
            body.push_str(&format!("\n\n**Full Changelog**: {}", url));
        }

        self.git_provider.release(params.app_name.as_str(), &new_tag, &body)?;

        Ok(AppReleaseUseCaseResponse {
            tag: new_tag,
            body
        })
    }
}
