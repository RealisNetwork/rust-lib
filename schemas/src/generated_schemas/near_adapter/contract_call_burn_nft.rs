// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterContractCallBurnNftParams {
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "approvalId")]
    pub approval_id: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}
impl Schema for NearAdapterContractCallBurnNftParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"tokenId\":{\"type\":\"string\"},\"approvalId\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"tokenId\"]}") . unwrap ()
    }
}
impl Agent for NearAdapterContractCallBurnNftParams {
    fn topic() -> &'static str {
        "near-adapter_contract_callBurnNft"
    }
    fn method() -> &'static str {
        "contract_callBurnNft"
    }
    fn agent() -> &'static str {
        "near-adapter"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterContractCallBurnNftReturns(pub bool);
impl Schema for NearAdapterContractCallBurnNftReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for NearAdapterContractCallBurnNftReturns {
    fn topic() -> &'static str {
        "near-adapter_contract_callBurnNft"
    }
    fn method() -> &'static str {
        "contract_callBurnNft"
    }
    fn agent() -> &'static str {
        "near-adapter"
    }
}
