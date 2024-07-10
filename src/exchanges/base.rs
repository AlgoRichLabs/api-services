use anyhow::{anyhow, Error};
use reqwest;
use reqwest::header::HeaderMap;
use std::collections::HashMap;
pub trait BaseExchange {}

// Every exchange api wrapper needs to send requests.
pub struct RestClient {
    client: reqwest::Client,
}

impl RestClient {
    pub fn new() -> Self {
        RestClient {
            client: reqwest::Client::new(),
        }
    }

    pub async fn send_request(
        &self,
        method: &str,
        url: &str,
        headers: Option<HeaderMap>,
        body: Option<HashMap<String, String>>,
    ) -> Result<reqwest::Response, Error> {
        let client = &self.client;
        let mut req = match method {
            "GET" => client.get(url),
            "POST" => client.post(url),
            "DELETE" => client.delete(url),
            _ => return Err(anyhow!("Invalid HTTP method: {}", method)),
        };

        if let Some(h) = headers {
            req = req.headers(h);
        }

        if let Some(b) = body {
            req = req.json(&b);
        }

        match req.send().await {
            Ok(res) => Ok(res),
            Err(e) => Err(anyhow!("Request error:{}", e)),
        }
    }
}
