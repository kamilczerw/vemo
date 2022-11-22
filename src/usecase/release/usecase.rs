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
        let tag = self.release_data_provider.find_latest_version(&params.app_name)?;

        let new_tag = match tag.clone() {
            Some(latest) => latest.bump_v2(&params.component),
            None => Tag::new_with_format(self.format.as_str(), &params.app_name, DEFAULT_VERSION)
        };

        let app_config = self.config_data_provider
            .get_app_config(params.app_name.as_str())?;

        let formatted_commits = self.get_formatted_commits(&tag, app_config.path)?;

        let mut body = String::from("## What's Changed\n\n");
        body.push_str(&formatted_commits);

        if let Ok(Some(url)) = self.release_data_provider.compare_url(&tag, &new_tag) {
            body.push_str(&format!("\n\n**Full Changelog**: {}", url));
        }

        self.release_data_provider.release(params.app_name.as_str(), &new_tag, &body)?;

        Ok(AppReleaseUseCaseResponse {
            tag: new_tag,
            body
        })
    }
}

impl AppReleaseUseCase {
    fn get_formatted_commits(&self, tag: &Option<Tag>, path: Option<String>) -> Result<String, AppReleaseUseCaseError> {
        let commits = self.release_data_provider.get_commits(tag, path)?;

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
