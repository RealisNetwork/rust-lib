// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for TransactionsBalanceGetBalancesByUserIdParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(TransactionsBalanceGetBalancesByUserIdParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct TransactionsBalanceGetBalancesByUserIdParams;
impl Schema for TransactionsBalanceGetBalancesByUserIdParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetBalancesByUserIdReturns {
    #[serde(rename = "ETH")]
    pub eth: String,
    #[serde(rename = "LIS")]
    pub lis: String,
}
impl Schema for TransactionsBalanceGetBalancesByUserIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"ETH\":{\"type\":\"string\"},\"LIS\":{\"type\":\"string\"}},\"required\":[\"ETH\",\"LIS\"]}")
    }
}