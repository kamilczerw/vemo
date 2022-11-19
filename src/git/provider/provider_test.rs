use crate::cfg::Config;
use crate::git::model::repo::Repo;
use crate::git::provider;
use crate::git::provider::{CliProvider, github};

#[test]
fn new_should_return_provider() {
    let cli = CliProvider::default();
    let config = Config::default();
    let git_provider = provider::new(&cli, &config);

    assert!(git_provider.is_ok());
}

#[test]
fn when_repo_provider_is_github_then_github_provider_should_be_returned() {
    let cli = CliProvider {
        repo: Repo {
            raw_url: "git@github.com:kamilczerw/vemo.git".to_string(),
            repo_name: "kamilczerw/vemo".to_string(),
            provider: provider::Provider::Github,
            repo_type: Default::default()
        }
    };
    let config = Config::default();
    let git_provider = provider::new(&cli, &config).unwrap();

    let git_provider = git_provider.downcast_ref::<github::Provider>();

    assert!(git_provider.is_some());
}

#[test]
fn when_repo_provider_is_unknown_then_no_provider_should_be_returned() {
    let cli = CliProvider {
        repo: Repo {
            raw_url: "git@github.com:kamilczerw/vemo.git".to_string(),
            repo_name: "kamilczerw/vemo".to_string(),
            provider: provider::Provider::Unknown,
            repo_type: Default::default()
        }
    };
    let config = Config::default();
     if let Err(error) = provider::new(&cli, &config) {
            assert_eq!(error.to_string(), "Git provider not supported: unknown");
        } else {
         panic!("Error should be returned");
     }
}
