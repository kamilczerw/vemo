use std::collections::HashMap;
use mockall::mock;
use crate::git::client::github::GithubClient;
use crate::lib::http::MockClient;
use mockall::*;

use crate::git::client::Git;
// use crate::git::client::MockGit;


// mock! {
//     // Client {}
//
//     // impl HttpClient for Client {
//     //     pub fn post(&self, url: &str) -> RequestBuilder;
//     // }
// }

mock! {
    Git {}
}

#[test]
fn create_a_release_should_call_github_api() {
    let github = GithubClient::init(
        Box::new(MockClient::new()),
        "https://api.github.com".to_string(),
        Git::new(),
        HashMap::new()
    ).unwrap();
    // let name = "name".to_string();
    // let tag = Release {
    //     raw: "tag".to_string(),
    //     semantic: "semantic".to_string(),
    // };
    // let description = "description".to_string();
    // github.create_release(name, tag, description).unwrap();
}
