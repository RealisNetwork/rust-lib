// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterContractCallNftLockParams {
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "approvalId")]
    pub approval_id: Option<String>,
}
impl Schema for NearAdapterContractCallNftLockParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"tokenId\":{\"type\":\"string\"},\"approvalId\":{\"type\":\"string\"}},\"required\":[\"tokenId\"]}") . unwrap ()
    }
}
impl Agent for NearAdapterContractCallNftLockParams {
    fn topic() -> &'static str {
        "near-adapter_contract_callNftLock"
    }
    fn method() -> &'static str {
        "contract_callNftLock"
    }
    fn agent() -> &'static str {
        "near-adapter"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterContractCallNftLockReturns(pub bool);
impl Schema for NearAdapterContractCallNftLockReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for NearAdapterContractCallNftLockReturns {
    fn topic() -> &'static str {
        "near-adapter_contract_callNftLock"
    }
    fn method() -> &'static str {
        "contract_callNftLock"
    }
    fn agent() -> &'static str {
        "near-adapter"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
