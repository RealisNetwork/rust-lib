// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulAdapterWalletGetBalanceByAddressParams {
    #[serde(rename = "address", deserialize_with = "deserialize_to_string")]
    pub address: String,
}
impl Schema for SoulAdapterWalletGetBalanceByAddressParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"address\":{\"type\":\"string\"}},\"required\":[\"address\"]}") . unwrap ()
    }
}
impl Agent for SoulAdapterWalletGetBalanceByAddressParams {
    fn topic() -> &'static str {
        "soul-adapter_wallet_getBalanceByAddress"
    }
    fn method() -> &'static str {
        "wallet_getBalanceByAddress"
    }
    fn agent() -> &'static str {
        "soul-adapter"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulAdapterWalletGetBalanceByAddressReturns(pub String);
impl Schema for SoulAdapterWalletGetBalanceByAddressReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"string\"}")
    }
}
impl Agent for SoulAdapterWalletGetBalanceByAddressReturns {
    fn topic() -> &'static str {
        "soul-adapter_wallet_getBalanceByAddress"
    }
    fn method() -> &'static str {
        "wallet_getBalanceByAddress"
    }
    fn agent() -> &'static str {
        "soul-adapter"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
