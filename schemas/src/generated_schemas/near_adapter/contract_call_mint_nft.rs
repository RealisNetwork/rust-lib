// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterContractCallMintNftParams {
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "metadata")]
    pub metadata: Option<String>,
}
impl Schema for NearAdapterContractCallMintNftParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"tokenId\":{\"type\":\"string\"},\"metadata\":{\"type\":\"string\"}},\"required\":[\"tokenId\"]}")
    }
}
impl Agent for NearAdapterContractCallMintNftParams {
    fn topic() -> &'static str {
        "near-adapter_contract_callMintNft"
    }
    fn method() -> &'static str {
        "contract_callMintNft"
    }
    fn agent() -> &'static str {
        "near-adapter"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterContractCallMintNftReturns(pub bool);
impl Schema for NearAdapterContractCallMintNftReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for NearAdapterContractCallMintNftReturns {
    fn topic() -> &'static str {
        "near-adapter_contract_callMintNft"
    }
    fn method() -> &'static str {
        "contract_callMintNft"
    }
    fn agent() -> &'static str {
        "near-adapter"
    }
}
