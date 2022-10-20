// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterContractCallNftUnlockParams {
    #[serde(rename = "tokenId", deserialize_with = "deserialize_to_string")]
    pub token_id: String,
}
impl Schema for NearAdapterContractCallNftUnlockParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"tokenId\":{\"type\":\"string\"}},\"required\":[\"tokenId\"]}") . unwrap ()
    }
}
impl Agent for NearAdapterContractCallNftUnlockParams {
    fn topic() -> &'static str {
        "near-adapter_contract_callNftUnlock"
    }
    fn method() -> &'static str {
        "contract_callNftUnlock"
    }
    fn agent() -> &'static str {
        "near-adapter"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterContractCallNftUnlockReturns(pub bool);
impl Schema for NearAdapterContractCallNftUnlockReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for NearAdapterContractCallNftUnlockReturns {
    fn topic() -> &'static str {
        "near-adapter_contract_callNftUnlock"
    }
    fn method() -> &'static str {
        "contract_callNftUnlock"
    }
    fn agent() -> &'static str {
        "near-adapter"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
