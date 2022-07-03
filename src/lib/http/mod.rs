pub(crate) mod client;

use std::collections::HashMap;
use serde_json::Value;
use mockall::*;
pub use client::HttpClient;

#[derive(Debug)]
pub enum HttpClientError {
    RequestError(reqwest::Error)
}

#[automock]
pub trait Client {
    fn get(&self, path: &str, params: HashMap<String, String>) -> Result<String, HttpClientError>;

    fn post(&self, path: &str, body: Value) -> Result<Value, HttpClientError>;
}
