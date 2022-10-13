// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterContractCallNftUnlockAndTransferParams {
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for NearAdapterContractCallNftUnlockAndTransferParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"tokenId\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"tokenId\",\"userId\"]}") . unwrap ()
    }
}
impl Agent for NearAdapterContractCallNftUnlockAndTransferParams {
    fn topic() -> &'static str {
        "near-adapter_contract_callNftUnlockAndTransfer"
    }
    fn method() -> &'static str {
        "contract_callNftUnlockAndTransfer"
    }
    fn agent() -> &'static str {
        "near-adapter"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterContractCallNftUnlockAndTransferReturns(pub bool);
impl Schema for NearAdapterContractCallNftUnlockAndTransferReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for NearAdapterContractCallNftUnlockAndTransferReturns {
    fn topic() -> &'static str {
        "near-adapter_contract_callNftUnlockAndTransfer"
    }
    fn method() -> &'static str {
        "contract_callNftUnlockAndTransfer"
    }
    fn agent() -> &'static str {
        "near-adapter"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
