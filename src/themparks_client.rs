use crate::prelude::*;

#[derive(Clone)]
pub struct ThemeparksClient {
    http_client: reqwest::Client,
    base_url: &'static str,
}

impl ThemeparksClient {
    pub fn new() -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap_or_default();

        Self {
            base_url: BASE_URL,
            http_client,
        }
    }

    pub async fn get(&self, path: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = self.url(path);
        println!("fetching {}", url);
        let response = self.http_client.get(url).send().await?;

        Ok(response)
    }

    fn url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path)
    }
}
