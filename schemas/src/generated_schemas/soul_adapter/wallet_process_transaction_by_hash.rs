// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulAdapterWalletProcessTransactionByHashParams {
    #[serde(rename = "blockNumber")]
    pub block_number: f64,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "hash")]
    pub hash: String,
}
impl Schema for SoulAdapterWalletProcessTransactionByHashParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"blockNumber\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"},\"hash\":{\"type\":\"string\"}},\"required\":[\"hash\",\"blockNumber\",\"currency\"]}") . unwrap ()
    }
}
impl Agent for SoulAdapterWalletProcessTransactionByHashParams {
    fn topic() -> &'static str {
        "soul-adapter_wallet_processTransactionByHash"
    }
    fn method() -> &'static str {
        "wallet_processTransactionByHash"
    }
    fn agent() -> &'static str {
        "soul-adapter"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulAdapterWalletProcessTransactionByHashReturns(pub bool);
impl Schema for SoulAdapterWalletProcessTransactionByHashReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for SoulAdapterWalletProcessTransactionByHashReturns {
    fn topic() -> &'static str {
        "soul-adapter_wallet_processTransactionByHash"
    }
    fn method() -> &'static str {
        "wallet_processTransactionByHash"
    }
    fn agent() -> &'static str {
        "soul-adapter"
    }
}
