// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterContractCallWithdrawParams {
    #[serde(
        rename = "recipientAddress",
        deserialize_with = "deserialize_to_string"
    )]
    pub recipient_address: String,
    #[serde(rename = "signature", deserialize_with = "deserialize_to_string")]
    pub signature: String,
    #[serde(rename = "amount", deserialize_with = "deserialize_to_string")]
    pub amount: String,
}
impl Schema for NearAdapterContractCallWithdrawParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"recipientAddress\":{\"type\":\"string\"},\"signature\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"}},\"required\":[\"recipientAddress\",\"amount\",\"signature\"]}") . unwrap ()
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
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAdapterContractCallWithdrawReturns {
    #[serde(rename = "balance", deserialize_with = "deserialize_to_string")]
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
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
