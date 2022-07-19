// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterContractCallTransferParams {
    #[serde(rename = "recipientUserId")]
    pub recipient_user_id: String,
    #[serde(rename = "amount")]
    pub amount: String,
}
impl Schema for NearAdapterContractCallTransferParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"recipientUserId\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"}},\"required\":[\"recipientUserId\",\"amount\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterContractCallTransferReturns {
    #[serde(rename = "balance")]
    pub balance: String,
}
impl Schema for NearAdapterContractCallTransferReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"balance\":{\"type\":\"string\"}},\"required\":[\"balance\"]}")
    }
}