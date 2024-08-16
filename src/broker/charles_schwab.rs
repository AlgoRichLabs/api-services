use std::collections::HashMap;
use std::fmt::format;
use reqwest::header::{HeaderMap, HeaderValue};
use base64::{Engine, engine::general_purpose::STANDARD};
use reqwest::header;

pub struct CharlesSchwab {
    app_key: String,
    secret: String,
    base_url: String,
    auth_url: String,
    is_sandbox: bool,
}

struct Payload {
    grant_type: String,
    code: String,
    redirect_uri: String,
}

struct Response {}

impl CharlesSchwab {
    pub fn new(configs: HashMap<String, String>) -> Self {
        let app_key: String = configs.get("app_key").unwrap().to_string();
        let secret: String = configs.get("secret").unwrap().to_string();
        let base_url: String = "https://api.schwabapi.com/trader/v1".to_string();
        let is_sandbox: bool = configs
            .get("is_sandbox")
            .and_then(|s| s.parse::<bool>().ok())
            .unwrap_or(false);

        let auth_url: String = format!(
            "https://api.schwabapi.com/v1/oauth/authorize?client_id={:?}&redirect_uri=https://127.0.0.1",
            app_key
        );

        CharlesSchwab {
            app_key,
            secret,
            base_url,
            auth_url,
            is_sandbox,
        }
    }

    async fn construct_headers_and_payload(&self, returned_url: String) -> (HeaderMap, Payload) {
        // TODO: extract response_code from the returned_url
        let response_code = "".to_string();
        let credentials: String = format!("{:?}:{:?}", self.app_key, self.secret);
        let encoded_credentials = format!("Basic {:?}", STANDARD.encode(&credentials));

        let mut header: HeaderMap = HeaderMap::new();
        header.insert("Authorization", encoded_credentials.parse::<HeaderValue>().unwrap());

        let payload = Payload {
            grant_type: "authorization_code".to_string(),
            code: response_code,
            redirect_uri: "https://127.0.0.1".to_string(),
        };

        (header, payload)
    }



}
