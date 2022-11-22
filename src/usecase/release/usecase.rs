use semver::Version;
use crate::git::model::tag::Tag;
use crate::usecase::release::{AppReleaseUseCase, AppReleaseUseCaseError, AppReleaseUseCaseRequest, AppReleaseUseCaseResponse, DEFAULT_VERSION};
use crate::usecase::UseCase;

/// # Summary
///
/// Use case for releasing an application for a given component.
///
/// # Params
///  - `app_name` - name of the application to release
///  - `component` - version component to be bumped
impl UseCase<AppReleaseUseCaseRequest, AppReleaseUseCaseResponse, AppReleaseUseCaseError> for AppReleaseUseCase {
    fn execute(&self, params: AppReleaseUseCaseRequest) -> Result<AppReleaseUseCaseResponse, AppReleaseUseCaseError> {
        let tag = self.get_latest_tag(&params.app_name)?;
        let new_tag = tag.clone().bump_v2(&params.component);

        let app_config = self.config_data_provider
            .get_app_config(params.app_name.as_str())?;

        let formatted_commits = self.get_formatted_commits(&tag, app_config.path)?;

        let mut body = String::from("## What's Changed\n\n");
        body.push_str(&formatted_commits);

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

impl AppReleaseUseCase {
    fn get_latest_tag(&self, app_name: &str) -> Result<Tag, AppReleaseUseCaseError> {
        let latest_version = self.git_provider.find_latest_version(app_name)?;

        let version = match latest_version {
            Some(version) => version,
            None => DEFAULT_VERSION
        };
        let tag = Tag::new_with_format(self.format.as_str(), app_name, version);

        Ok(tag)
    }

    fn get_formatted_commits(&self, tag: &Tag, path: Option<String>) -> Result<String, AppReleaseUseCaseError> {
        let commits = self.git_provider.get_commits(tag, path)?;

        if commits.is_empty() {
            return Err(AppReleaseUseCaseError::NoChanges)
        }

        let mut body = String::from("");
        for commit in commits {
            body.push_str(&format!("* {} by {}\n", commit.message, commit.author));
        };

        Ok(body)
    }
}
