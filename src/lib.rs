pub mod broker;
pub mod constants;
pub mod exchange;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::exchange::okx::OkxExchange;
    use serde_json::Value;
    use std::collections::HashMap;
    use std::fs;
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
    async fn test_okx_get_requests() {
        let configs: HashMap<String, String> = read_configs("configs.json", "okx_account");
        let okx_exchange = OkxExchange::new(&configs);

        println!("Testing get_account_info method.");
        match okx_exchange.get_account_info().await {
            Ok(account_info) => println!("Account info: {:?}", account_info),
            Err(e) => println!("Error getting account info: {:?}", e),
        }

        println!("Testing get_balances method.");
        match okx_exchange.get_balances().await {
            Ok(balances) => {
                println!("Balances: {:?}", balances);
            }
            Err(e) => println!("Error getting balances: {:?}", e),
        }

        println!("Testing get_positions_info method.");
        match okx_exchange.get_positions_info().await {
            Ok(positions_info) => {
                println!("Positions Info: {:?}", positions_info);
            }
            Err(e) => println!("Error getting positions info: {:?}", e),
        }

        println!("Testing get_position_info method.");
        match okx_exchange.get_position_info("XRP-USD-240927").await {
            Ok(positions_info) => {
                println!("Position Info: {:?}", positions_info)
            }
            Err(e) => println!("Error getting position info: {:?}", e),
        }
    }
}
