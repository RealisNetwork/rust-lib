// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterContractCallTransferParams {
    #[serde(rename = "receiverId")]
    pub receiver_id: String,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    #[serde(rename = "amount")]
    pub amount: String,
}
impl Schema for NearAdapterContractCallTransferParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"receiverId\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"}},\"required\":[\"receiverId\",\"amount\"]}") . unwrap ()
    }
}
impl Agent for NearAdapterContractCallTransferParams {
    fn topic() -> &'static str {
        "near-adapter_contract_callTransfer"
    }
    fn method() -> &'static str {
        "contract_callTransfer"
    }
    fn agent() -> &'static str {
        "near-adapter"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterContractCallTransferReturns(pub bool);
impl Schema for NearAdapterContractCallTransferReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for NearAdapterContractCallTransferReturns {
    fn topic() -> &'static str {
        "near-adapter_contract_callTransfer"
    }
    fn method() -> &'static str {
        "contract_callTransfer"
    }
    fn agent() -> &'static str {
        "near-adapter"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
