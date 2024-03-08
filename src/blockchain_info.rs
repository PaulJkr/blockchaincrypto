use dotenv;
use reqwest;
use tokio;

use crate::blockchain_status::BlockchainStatus;
use crate::blockchain_address::BlockchainAddress;
use crate::blockchain_transaction::BlockchainTransaction;

const HOST_ROOT: &str = "https://btcbook.nownodes.io/api/";

#[tokio::main]
pub async fn send_request(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("api-key", dotenv::var("API_KEY").expect("Could not find key: API_KEY"))
        .send()
        .await?;
    let body = response.text().await?;
    Ok(body)
}

pub fn blockchain_status_request() -> Result<BlockchainStatus, Box<dyn std::error::Error>> {
    let response = send_request(HOST_ROOT)?;
    let status = serde_json::from_str(&response)?;
    Ok(status)
}

pub fn blockchain_address_request(address: &str) -> Result<BlockchainAddress, Box<dyn std::error::Error>> {
    let url = [HOST_ROOT, "v2/address/", address].join("");
    let response = send_request(&url)?;
    let address_info = serde_json::from_str(&response)?;
    Ok(address_info)
}

#[allow(dead_code)]
pub fn blockchain_transaction_request(transaction: &str) -> Result<BlockchainTransaction, Box<dyn std::error::Error>> {
    let url = [HOST_ROOT, "v2/tx/", transaction].join("");
    let response = send_request(&url)?;
    let transaction_info = serde_json::from_str(&response)?;
    Ok(transaction_info)
}
