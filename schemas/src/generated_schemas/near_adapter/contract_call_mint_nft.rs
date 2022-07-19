// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterContractCallMintNftParams {
    #[serde(rename = "recipientUserId")]
    pub recipient_user_id: String,
    #[serde(rename = "nftMetadata")]
    pub nft_metadata: String,
}
impl Schema for NearAdapterContractCallMintNftParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"recipientUserId\":{\"type\":\"string\"},\"nftMetadata\":{\"type\":\"string\"}},\"required\":[\"recipientUserId\",\"nftMetadata\"]}")
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
pub struct NearAdapterContractCallMintNftReturns {
    #[serde(rename = "nftId")]
    pub nft_id: String,
}
impl Schema for NearAdapterContractCallMintNftReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"nftId\":{\"type\":\"string\"}},\"required\":[\"nftId\"]}")
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
