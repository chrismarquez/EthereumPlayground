use std::error::Error;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::Params::{GetBalance, GetBlockParams};

const ALCHEMY_URL: &str = "https://eth-sepolia.g.alchemy.com/v2/dPPYcqq7wCDll07cUvnWQNtnBtx19EFN";

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Params {
    GetBlockParams((String, bool)),
    GetBalance((String, String))
}

#[derive(Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    id: u32,
    params: Params
}

impl JsonRpcRequest {
    fn new_get_block_by_num(block: &str, is_full: bool) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id: 0,
            method: "eth_getBlockByNumber".into(),
            params: GetBlockParams((block.into(), is_full))
        }
    }
    fn new_get_latest_balance(account: &str) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id: 0,
            method: "eth_getBalance".into(),
            params: GetBalance((account.into(), "latest".into()))
        }
    }

    async fn send(&self, client: &Client) -> Result<String, Box<dyn Error>> {
        let response = client.post(ALCHEMY_URL)
            .json(&self)
            .send()
            .await?;
        let bytes = response.bytes().await?;
        let result = std::str::from_utf8(&bytes)?.to_string();
        Ok(result)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let block = "0xb443";
    let request = JsonRpcRequest::new_get_block_by_num(block, true);
    let response = request.send(&client).await?;
    println!("{}", response);
    let request = JsonRpcRequest::new_get_latest_balance("0xe5cB067E90D5Cd1F8052B83562Ae670bA4A211a8");
    let response = request.send(&client).await?;
    println!("{}", response);
    Ok(())
}
