// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterWalletGetUserByAccountIdParams {
    #[serde(rename = "accountId")]
    pub account_id: String,
}
impl Schema for NearAdapterWalletGetUserByAccountIdParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"accountId\":{\"type\":\"string\"}},\"required\":[\"accountId\"]}") . unwrap ()
    }
}
impl Agent for NearAdapterWalletGetUserByAccountIdParams {
    fn topic() -> &'static str {
        "near-adapter_wallet_getUserByAccountId"
    }
    fn method() -> &'static str {
        "wallet_getUserByAccountId"
    }
    fn agent() -> &'static str {
        "near-adapter"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterWalletGetUserByAccountIdReturns(pub String);
impl Schema for NearAdapterWalletGetUserByAccountIdReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"string\"}")
    }
}
impl Agent for NearAdapterWalletGetUserByAccountIdReturns {
    fn topic() -> &'static str {
        "near-adapter_wallet_getUserByAccountId"
    }
    fn method() -> &'static str {
        "wallet_getUserByAccountId"
    }
    fn agent() -> &'static str {
        "near-adapter"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
