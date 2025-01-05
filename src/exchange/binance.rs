use crate::exchange::base::RestClient;
use anyhow::{anyhow, Error};
use hmac::{Hmac, Mac};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use sha2::Sha256;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use url::form_urlencoded;

pub struct BinanceExchange {
    key: String,
    secret: String,
    pm_base_url: String,
    rest_client: RestClient,
}

#[derive(Debug, Deserialize, Serialize)]
enum DataValue {
    ValueString(String),
    ValueMap(HashMap<String, DataValue>),
    ValueVector(Vec<DataValue>),
}

impl DataValue {
    fn from_json(value: Value) -> Self {
        match value {
            Value::String(s) => DataValue::ValueString(s),
            Value::Object(map) => {
                let parsed_map = map
                    .into_iter()
                    .map(|(k, v)| (k, DataValue::from_json(v)))
                    .collect();
                DataValue::ValueMap(parsed_map)
            }
            Value::Array(arr) => {
                let parsed_vec = arr.into_iter().map(|v| DataValue::from_json(v)).collect();
                DataValue::ValueVector(parsed_vec)
            }
            Value::Number(num) => DataValue::ValueString(num.to_string()),
            _ => {
                println!("Json value: {}", value);
                panic!("Unsupported JSON type!")
            }
        }
    }
}

impl BinanceExchange {
    pub fn new(configs: &HashMap<String, String>) -> Self {
        let key = configs.get("key").unwrap().to_string();
        let secret = configs.get("secret").unwrap().to_string();
        let pm_base_url = "https://papi.binance.com".to_string(); // Portfolio margin account api base url.
        let rest_client = RestClient::new();

        BinanceExchange {
            key,
            secret,
            pm_base_url,
            rest_client,
        }
    }

    pub async fn get_account_info(&self) -> Result<HashMap<String, String>, Error> {
        let endpoint = "/papi/v1/account";
        let response = self.send_request("GET", endpoint, None).await?;

        if let DataValue::ValueMap(map) = response {
            let mut string_map: HashMap<String, String> = HashMap::new();
            for (key, value) in map {
                if let DataValue::ValueString(val) = value {
                    string_map.insert(key, val);
                } else {
                    return Err(anyhow!("Value for key {} is not a ValueString", key));
                }
            }
            Ok(string_map)
        } else {
            Err(anyhow!("DataValue is not a ValueMap"))
        }
    }

    fn generate_signature(&self, query_string: &str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret.as_bytes()).unwrap();
        mac.update(query_string.as_bytes());
        let signature = mac.finalize().into_bytes();
        hex::encode(signature)
    }

    fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/json;charset=utf-8".parse().unwrap(),
        );
        headers.insert("X-MBX-APIKEY", self.key.parse().unwrap());
        headers
    }

    async fn send_request(
        &self,
        method: &str,
        endpoint: &str,
        params: Option<HashMap<String, String>>,
    ) -> Result<DataValue, Error> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let query_string = match &params {
            Some(map) => format!(
                "{}&timestamp={}",
                form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(map)
                    .finish(),
                timestamp
            ),
            None => format!("timestamp={}", timestamp),
        };
        let encoded_signature = self.generate_signature(&query_string);
        let headers: HeaderMap = self.get_headers();
        let url = format!(
            "{}{}?{}&signature={}",
            self.pm_base_url, endpoint, query_string, encoded_signature
        );
        let response = self
            .rest_client
            .send_request(method, &url, Some(headers), params)
            .await?;
        let status = response.status();
        let response_text = response.text().await?;
        let response_json_value = match from_str(&response_text) {
            Ok(json) => json,
            Err(err) => {
                return Err(anyhow!(
                    "Failed to parse string to json: {}. Error: {}.",
                    response_text,
                    err
                ))
            }
        };

        let data_value = DataValue::from_json(response_json_value);
        if status.is_success() {
            println!("Json string: {}", response_text);
            Ok(data_value)
        } else {
            Err(anyhow!("API error: {}", response_text))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_configs;

    fn init_client() -> BinanceExchange {
        let config = read_configs("configs.json", "binance_account");
        BinanceExchange::new(&config)
    }

    #[tokio::test]
    async fn test_get_account_info() {
        let client = init_client();
        match client.get_account_info().await {
            Ok(info) => println!("Account info: {:?}", info),
            Err(e) => println!("test_get_account_info error: {:?}", e),
        }
    }
}
