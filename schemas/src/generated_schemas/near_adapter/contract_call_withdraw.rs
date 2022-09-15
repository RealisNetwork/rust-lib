// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterContractCallWithdrawParams {
    #[serde(rename = "recipientAddress")]
    pub recipient_address: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "signature")]
    pub signature: String,
}
impl Schema for NearAdapterContractCallWithdrawParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"recipientAddress\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"},\"signature\":{\"type\":\"string\"}},\"required\":[\"recipientAddress\",\"amount\",\"signature\"]}")
    }
}
impl Agent for NearAdapterContractCallWithdrawParams {
    fn topic() -> &'static str {
        "near-adapter_contract_callWithdraw"
    }
    fn method() -> &'static str {
        "contract_callWithdraw"
    }
    fn agent() -> &'static str {
        "near-adapter"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterContractCallWithdrawReturns {
    #[serde(rename = "balance")]
    pub balance: String,
}
impl Schema for NearAdapterContractCallWithdrawReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"balance\":{\"type\":\"string\"}},\"required\":[\"balance\"]}")
    }
}
impl Agent for NearAdapterContractCallWithdrawReturns {
    fn topic() -> &'static str {
        "near-adapter_contract_callWithdraw"
    }
    fn method() -> &'static str {
        "contract_callWithdraw"
    }
    fn agent() -> &'static str {
        "near-adapter"
    }
}
