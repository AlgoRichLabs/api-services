use serde_json::Value;
use std::collections::HashMap;
use std::fs;

pub fn read_configs(file_path: &str, account_name: &str) -> HashMap<String, String> {
    let config_data = fs::read_to_string(file_path).expect("Unable to read config file.");
    let accounts: Vec<Value> = serde_json::from_str(&config_data).expect("Invalid JSON format.");
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
