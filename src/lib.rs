use log::debug;
use serde::Deserialize;
use serde_json::Value;

use crate::http::http_send;
use crate::types::{JsonRpc, JsonRpcRequest};
use crate::utils::random_number;

pub mod types;
mod utils;
mod http;

/// BlockScanner
#[derive(Default, Deserialize, Debug, Clone)]
pub struct BlockScanner {
    /// RPC URLs
    pub url: Vec<String>,
    /// re-try quest
    pub retry: Retry,
    /// chain Id
    pub id: String,
}

/// Retry
#[derive(Deserialize, Debug, Clone)]
pub struct Retry(u32);

impl Default for Retry {
    fn default() -> Self {
        Retry(3)
    }
}

impl BlockScanner {
    pub async fn get_block_latest_number(&self) -> Option<JsonRpc> {
        let query = JsonRpcRequest {
            method: "eth_blockNumber".to_string(),
            params: Value::Array(vec![]),
            id: self.id.parse().unwrap(),
            jsonrpc: "2.0".to_string(),
        };
        debug!("Query {:?}", query);
        http_send(self.url.as_ref(), self.retry.0, query).await
    }

    pub async fn get_transactions_for_block(&self, number: &str) -> Option<JsonRpc> {
        let query = JsonRpcRequest {
            method: "eth_getBlockByNumber".to_string(),
            params: Value::Array(vec![Value::String(number.to_string()), Value::Bool(true)]),
            id: self.id.parse().unwrap(),
            jsonrpc: "2.0".to_string(),
        };
        debug!("Query {:?}", query);
        http_send(self.url.as_ref(), self.retry.0, query).await
    }

    pub async fn get_transaction_receipt_for_hash(&self, hash: &str) -> Option<JsonRpc> {
        let query = JsonRpcRequest {
            method: "eth_getTransactionReceipt".to_string(),
            params: Value::Array(vec![Value::String(hash.to_string())]),
            id: self.id.parse().unwrap(),
            jsonrpc: "2.0".to_string(),
        };
        debug!("Query {:?}", query);
        http_send(self.url.as_ref(), self.retry.0, query).await
    }
}


#[cfg(test)]
mod tests { 
    use super::*;

    #[async_std::test]
    async fn test_get_latest_number() {
        let bs = BlockScanner {
            url: vec!["https://cloudflare-eth.com/".to_string(), "https://mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161".to_string()],
            retry: Default::default(),
            id: "1".to_string(),
        };

        assert_eq!(true, bs.get_block_latest_number().await.is_some())
    }

    #[async_std::test]
    async fn test_get_transactions_for_block() {
        let bs = BlockScanner {
            url: vec!["https://cloudflare-eth.com/".to_string(), "https://mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161".to_string()],
            retry: Default::default(),
            id: "1".to_string(),
        };
        assert_eq!(true, bs.get_transactions_for_block("0xe5b544").await.is_some());
    }

    #[async_std::test]
    async fn test_get_transactions_for_block_error() {
        let bs = BlockScanner {
            url: vec!["https://cloudflare-eth.com/".to_string(), "https://mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161".to_string()],
            retry: Default::default(),
            id: "1".to_string(),
        };
        assert_eq!(false, bs.get_transactions_for_block("0xe5a2h0").await.is_some());
    }

    #[async_std::test]
    async fn test_get_transaction_receipt_for_hash_error() {
        let bs = BlockScanner {
            url: vec!["https://cloudflare-eth.com/".to_string(), "https://mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161".to_string()],
            retry: Default::default(),
            id: "1".to_string(),
        };
        
        assert_eq!(false, bs.get_transaction_receipt_for_hash("0x80fdaa7f5f54cbe28b84f41afb9543cf0c9eb0d9f4b8a620c2fb5faf0b1c2810").await.is_none());
    }

    #[async_std::test]
    async fn test_get_transaction_receipt_for_hash() {
        let bs = BlockScanner {
            url: vec!["https://cloudflare-eth.com/".to_string(), "https://mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161".to_string()],
            retry: Default::default(),
            id: "1".to_string(),
        };
        assert_eq!(true, bs.get_transaction_receipt_for_hash("0x80fdaa7f5f54cbe28b84f41afb9543cf0c9eb0d9f4b8a620c2fb5faf0b1c2810").await.is_some());
    }
}
