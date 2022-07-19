// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulAdapterWalletGetUserIdByAddressParams {
    #[serde(rename = "address")]
    pub address: String,
}
impl Schema for SoulAdapterWalletGetUserIdByAddressParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"address\":{\"type\":\"string\"}},\"required\":[\"address\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulAdapterWalletGetUserIdByAddressReturns(String);
impl Schema for SoulAdapterWalletGetUserIdByAddressReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"string\"}")
    }
}