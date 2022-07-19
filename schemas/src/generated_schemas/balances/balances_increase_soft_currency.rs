// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalancesBalancesIncreaseSoftCurrencyParams {
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "amount")]
    pub amount: i32,
}
impl Schema for BalancesBalancesIncreaseSoftCurrencyParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"txId\":{\"type\":\"string\"},\"amount\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"amount\",\"txId\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalancesBalancesIncreaseSoftCurrencyReturns {
    #[serde(rename = "balance")]
    pub balance: i32,
}
impl Schema for BalancesBalancesIncreaseSoftCurrencyReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"balance\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"balance\"]}")
    }
}
