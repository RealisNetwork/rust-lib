// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceWalletWalletCreateWalletParams {
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
}
impl Schema for BinanceWalletWalletCreateWalletParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for BinanceWalletWalletCreateWalletParams {
    fn topic() -> &'static str {
        "binance-wallet_wallet_createWallet"
    }
    fn method() -> &'static str {
        "wallet_createWallet"
    }
    fn agent() -> &'static str {
        "binance-wallet"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceWalletWalletCreateWalletReturns {
    #[serde(rename = "privateKey", deserialize_with = "deserialize_to_string")]
    pub private_key: String,
    #[serde(rename = "address", deserialize_with = "deserialize_to_string")]
    pub address: String,
}
impl Schema for BinanceWalletWalletCreateWalletReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"privateKey\":{\"type\":\"string\"},\"address\":{\"type\":\"string\"}},\"required\":[\"privateKey\",\"address\"]}")
    }
}
impl Agent for BinanceWalletWalletCreateWalletReturns {
    fn topic() -> &'static str {
        "binance-wallet_wallet_createWallet"
    }
    fn method() -> &'static str {
        "wallet_createWallet"
    }
    fn agent() -> &'static str {
        "binance-wallet"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
