use anyhow::{anyhow, Error};
use reqwest;
use reqwest::header::HeaderMap;
use std::collections::HashMap;

use crate::constants::Side;
use crate::exchanges::exchange_types::FetchPositionParams;

pub trait BaseExchange {
    async fn get_ticker(&self, symbol: &str) -> Result<HashMap<String, String>, Error>;

    async fn fetch_positions(
        &self,
        params: FetchPositionParams,
    ) -> Result<Vec<HashMap<String, String>>, Error>;

    async fn fetch_balances(&self) -> Result<Vec<HashMap<String, String>>, Error>;

    async fn get_bbo_price(&self, symbol: &str, side: Side) -> Result<f64, Error>;
}

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

        let res = req.send().await?;
        Ok(res)
    }
}
