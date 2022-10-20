// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseWalletGetWalletByUserIdParams {
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
}
impl Schema for PurchaseWalletGetWalletByUserIdParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for PurchaseWalletGetWalletByUserIdParams {
    fn topic() -> &'static str {
        "purchase_wallet_getWalletByUserId"
    }
    fn method() -> &'static str {
        "wallet_getWalletByUserId"
    }
    fn agent() -> &'static str {
        "purchase"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseWalletGetWalletByUserIdReturns {
    #[serde(rename = "address", deserialize_with = "deserialize_to_string")]
    pub address: String,
}
impl Schema for PurchaseWalletGetWalletByUserIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"address\":{\"type\":\"string\"}},\"required\":[\"address\"]}")
    }
}
impl Agent for PurchaseWalletGetWalletByUserIdReturns {
    fn topic() -> &'static str {
        "purchase_wallet_getWalletByUserId"
    }
    fn method() -> &'static str {
        "wallet_getWalletByUserId"
    }
    fn agent() -> &'static str {
        "purchase"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
