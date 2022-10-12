// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceWalletWalletGetAddressByUserIdParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for BinanceWalletWalletGetAddressByUserIdParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for BinanceWalletWalletGetAddressByUserIdParams {
    fn topic() -> &'static str {
        "binance-wallet_wallet_getAddressByUserId"
    }
    fn method() -> &'static str {
        "wallet_getAddressByUserId"
    }
    fn agent() -> &'static str {
        "binance-wallet"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceWalletWalletGetAddressByUserIdReturns(pub String);
impl Schema for BinanceWalletWalletGetAddressByUserIdReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"string\"}")
    }
}
impl Agent for BinanceWalletWalletGetAddressByUserIdReturns {
    fn topic() -> &'static str {
        "binance-wallet_wallet_getAddressByUserId"
    }
    fn method() -> &'static str {
        "wallet_getAddressByUserId"
    }
    fn agent() -> &'static str {
        "binance-wallet"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
