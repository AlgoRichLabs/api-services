pub mod brokers;
pub mod constants;
pub mod errors;
pub mod exchanges;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::exchanges::base::BaseExchange;
    use crate::exchanges::okx::OkxExchange;
    use serde_json::Value;
    use std::collections::HashMap;
    use std::fs;
    use std::hash::Hash;
    use std::string::String;
    use tokio;

    fn read_configs(file_path: &str, account_name: &str) -> HashMap<String, String> {
        let config_data = fs::read_to_string(file_path).expect("Unable to read config file.");
        let accounts: Vec<Value> =
            serde_json::from_str(&config_data).expect("Invalid JSON format.");
        let mut configs: HashMap<String, String> = HashMap::new();
        for account in accounts {
            if account["name"] == account_name {
                if let Some(account_map) = account.as_object() {
                    for (key, value) in account_map {
                        if let Some(val_str) = value.as_str() {
                            configs.insert(key.clone(), val_str.to_string());
                        }
                    }
                    break;
                }
            }
        }

        if configs.is_empty() {
            panic!("Account not found.")
        }

        configs
    }

    #[tokio::test]
    async fn test_make_request() {
        println!("Testing test_make_request method.");
        let configs: HashMap<String, String> = read_configs("configs.json", "okx_account");
        let okx_exchange = OkxExchange::new(&configs);
        match okx_exchange.fetch_balances().await {
            Ok(balances) => {
                println!("Balances: {:?}", balances);
            }
            Err(e) => println!("Error fetching balances: {:?}", e),
        }
    }
}
