use std::collections::HashMap;
use reqwest::blocking::RequestBuilder;
use crate::lib::http::{Client, HttpClientError};
use reqwest::blocking::{Client as ReqClient, Response};
use reqwest::header::HeaderMap;
use serde_json::Value;

pub struct HttpClient {
    base_url: String,
    headers: HashMap<String, String>,
    client: ReqClient,
}

impl HttpClient {
    /// Create a new HttpClient
    /// @TODO: Auth should be changed to a struct witch implements different types of auth.
    ///       For now, it's just a string which adds `Authorization: token {token}` header if not empty.
    pub fn new(base_url: &str, auth: Option<String>) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("User-Agent".to_string(), "Vemo-Cli".to_string());
        if let Some(auth) = auth {
            headers.insert("Authorization".to_string(), format!("token {}", auth));
        }

        HttpClient {
            base_url: base_url.to_string(),
            headers,
            client: ReqClient::new(),
        }
    }

    fn get_call(&self, path: &str, params: HashMap<String, String>) -> Result<Response, HttpClientError> {
        let url = format!("{}/{}", self.base_url, path);
        let builder = self.client.get(&url).query(&params);
        let res = self.http_request(builder)?;

        Ok(res)
    }

    fn url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path)
    }

    fn http_request(&self, req: RequestBuilder) -> Result<Response, HttpClientError> {
        let mut req = req;
        for (key, value) in self.headers.iter() {
            req = req.header(key, value);
        }

        let res = req.send()
            .map_err(|e| HttpClientError::RequestError(e))?;

        Ok(res)
    }
}

impl Client for HttpClient {
    fn get(&self, path: &str, params: HashMap<String, String>) -> Result<String, HttpClientError> {
        // @TODO: it should return an object not a string.
        //        Maybe it should be a generic function which takes a type and returns a Result<T, HttpClientError>
        //        it should parse the response and return the object.
        self.get_call(path, params).map(|res| res.text().unwrap())
    }

    fn post(&self, path: &str, body: Value) -> Result<Value, HttpClientError> {
        todo!()
    }
}
