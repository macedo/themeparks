use crate::prelude::*;

pub struct Client {}

impl Client {
    pub fn new() -> Result<reqwest::Client, reqwest::Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::ACCEPT, header::HeaderValue::from_static("application/json"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            // definir timeouts
            .build()?;

        Ok(client)
    }
}