use crate::exchange::base::RestClient;
use anyhow::{anyhow, Error};
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use sha2::Sha256;
use std::collections::HashMap;
use std::string::String;
use url::form_urlencoded;

pub struct OkxExchange {
    key: String,
    secret: String,
    passphrase: String,
    base_url: String,
    is_demo: bool,
    rest_client: RestClient,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    code: String,
    msg: String,
    data: Vec<HashMap<String, DataValue>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum DataValue {
    ValueString(String),
    ValueVector(Vec<HashMap<String, DataValue>>),
}

impl Response {
    fn from_json(json: &str) -> Result<Self, Error> {
        match serde_json::from_str(json) {
            Ok(response) => Ok(response),
            Err(e) => Err(e.into()),
        }
    }
}

impl OkxExchange {
    pub fn new(configs: &HashMap<String, String>) -> Self {
        let key: String = configs.get("key").unwrap().to_string();
        let secret: String = configs.get("secret").unwrap().to_string();
        let passphrase: String = configs.get("passphrase").unwrap().to_string();
        let base_url: String = "https://www.okx.com".to_string();
        let is_demo: bool = configs
            .get("is_demo")
            .and_then(|s| s.parse::<bool>().ok())
            .unwrap_or(false);
        let rest_client: RestClient = RestClient::new();

        OkxExchange {
            key,
            secret,
            passphrase,
            base_url,
            is_demo,
            rest_client,
        }
    }

    // Get Methods
    pub async fn get_account_info(&self) -> Result<HashMap<String, String>, Error> {
        let endpoint: &str = "/api/v5/account/balance";
        let response = self.send_request("GET", endpoint, None).await?;
        if response.is_empty() {
            return Err(anyhow!("No data returned from the API"));
        }

        let mut result: HashMap<String, String> = HashMap::new();

        for (key, value) in &response[0] {
            if let DataValue::ValueString(string_value) = value {
                result.insert(key.clone(), string_value.clone());
            }
        }

        Ok(result)
    }

    pub async fn get_total_equity(&self) -> Result<f64, Error> {
        let account_info: HashMap<String, String> = self.get_account_info().await?;
        let total_equity: f64 = if let Some(equity_str) = account_info.get("totalEq") {
            equity_str.parse().unwrap()
        } else {
            return Err(anyhow!("Key error when getting total equity."));
        };

        Ok(total_equity)
    }

    pub async fn get_maintenance_margin_ratio(&self) -> Result<f64, Error> {
        let account_info: HashMap<String, String> = self.get_account_info().await?;
        let mmr: f64 = if let Some(mmr_string) = account_info.get("mmr") {
            mmr_string.parse().unwrap()
        } else {
            return Err(anyhow!("Key error when getting maintenance margin ratio."));
        };

        Ok(mmr)
    }

    pub async fn get_balances(&self) -> Result<Vec<HashMap<String, String>>, Error> {
        let endpoint: &str = "/api/v5/account/balance";
        let response = self.send_request("GET", endpoint, None).await?;
        if response.is_empty() {
            return Err(anyhow!("No data returned from the API"));
        }

        match &response[0].get("details") {
            Some(DataValue::ValueVector(vec)) => {
                let mut result: Vec<HashMap<String, String>> = Vec::new();
                for map in vec {
                    let mut balance_map = HashMap::new();
                    for (key, value) in map {
                        if let DataValue::ValueString(string_value) = value {
                            balance_map.insert(key.clone(), string_value.clone());
                        }
                    }
                    result.push(balance_map);
                }
                Ok(result)
            }
            _ => Ok(Vec::new()),
        }
    }

    pub async fn get_positions_info(&self) -> Result<Vec<HashMap<String, String>>, Error> {
        let endpoint: &str = "/api/v5/account/positions";
        let response = self.send_request("GET", endpoint, None).await?;
        let mut result: Vec<HashMap<String, String>> = Vec::new();
        for item in &response {
            let mut position_map: HashMap<String, String> = HashMap::new();
            for (key, value) in item {
                if let DataValue::ValueString(s) = value {
                    position_map.insert(key.clone(), s.clone());
                }
            }
            result.push(position_map);
        }
        Ok(result)
    }

    pub async fn get_position_info(
        &self,
        instrument_id: &str,
    ) -> Result<HashMap<String, String>, Error> {
        let endpoint: &str = "/api/v5/account/positions";
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("instId".to_string(), instrument_id.to_string());
        let response: Vec<HashMap<String, DataValue>> =
            self.send_request("GET", endpoint, Some(params)).await?;
        if response.is_empty() {
            return Ok(HashMap::new());
        }

        let mut result: HashMap<String, String> = HashMap::new();
        for (key, value) in &response[0] {
            if let DataValue::ValueString(s) = value {
                result.insert(key.clone(), s.clone());
            }
        }

        Ok(result)
    }

    // Helper Functions:
    // Signature definition is specific to the exchange
    fn generate_signature(
        &self,
        method: &str,
        end_point: &str,
        query_string: &str,
        body: &str,
    ) -> (String, String) {
        let timestamp = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let pre_hash = format!(
            "{}{}{}{}{}",
            timestamp, method, end_point, query_string, body
        );
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret.as_bytes()).unwrap();
        mac.update(pre_hash.as_bytes());
        let signature = mac.finalize().into_bytes();
        let encoded_signature = STANDARD.encode(&signature);
        (encoded_signature, timestamp)
    }

    // Headers definition are specific to the exchange
    fn get_headers(&self, signature: &str, timestamp: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("OK-ACCESS-KEY", self.key.parse().unwrap());
        headers.insert("OK-ACCESS-SIGN", signature.parse().unwrap());
        headers.insert("OK-ACCESS-TIMESTAMP", timestamp.parse().unwrap());
        headers.insert("OK-ACCESS-PASSPHRASE", self.passphrase.parse().unwrap());
        if self.is_demo {
            headers.insert("x-simulated-trading", "1".parse().unwrap());
        }
        headers
    }

    async fn send_request(
        &self,
        method: &str,
        endpoint: &str,
        params: Option<HashMap<String, String>>,
    ) -> Result<Vec<HashMap<String, DataValue>>, Error> {
        let (query_string, body_string, body) = match method {
            "GET" => {
                let qs = match &params {
                    Some(map) => format!(
                        "?{}",
                        form_urlencoded::Serializer::new(String::new())
                            .extend_pairs(map)
                            .finish()
                    ),
                    None => String::new(),
                };
                (qs, String::new(), None)
            }
            "POST" | "DELETE" => {
                let bs = match &params {
                    Some(map) => to_string(map).unwrap_or_else(|_| String::new()),
                    None => String::new(),
                };
                (String::new(), bs, params)
            }
            _ => return Err(anyhow!("Invalid Method.")),
        };

        let (encoded_signature, timestamp) =
            self.generate_signature(method, endpoint, &query_string, &body_string);

        let headers: HeaderMap = self.get_headers(&encoded_signature, &timestamp);
        let url = format!("{}{}{}", self.base_url, endpoint, query_string);
        let response = self
            .rest_client
            .send_request(method, &url, Some(headers), body)
            .await?;

        let status: StatusCode = response.status();
        let response_text = response.text().await?;
        let api_response: Response = Response::from_json(&response_text)?;

        if status.is_success() {
            Ok(api_response.data)
        } else {
            Err(anyhow!(
                "API error: {} - {}",
                api_response.code,
                api_response.msg
            ))
        }
    }
}
