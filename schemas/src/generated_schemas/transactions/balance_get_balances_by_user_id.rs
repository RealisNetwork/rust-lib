// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for TransactionsBalanceGetBalancesByUserIdParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(TransactionsBalanceGetBalancesByUserIdParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct TransactionsBalanceGetBalancesByUserIdParams;
impl Schema for TransactionsBalanceGetBalancesByUserIdParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for TransactionsBalanceGetBalancesByUserIdParams {
    fn topic() -> &'static str {
        "transactions_balance_getBalancesByUserId"
    }
    fn method() -> &'static str {
        "balance_getBalancesByUserId"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetBalancesByUserIdReturns {
    #[serde(rename = "WLIS")]
    pub wlis: String,
    #[serde(rename = "LIS")]
    pub lis: String,
    #[serde(rename = "TLIS")]
    pub tlis: String,
    #[serde(rename = "ETH")]
    pub eth: String,
}
impl Schema for TransactionsBalanceGetBalancesByUserIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"WLIS\":{\"type\":\"string\"},\"LIS\":{\"type\":\"string\"},\"TLIS\":{\"type\":\"string\"},\"ETH\":{\"type\":\"string\"}},\"required\":[\"ETH\",\"LIS\",\"WLIS\",\"TLIS\"]}")
    }
}
impl Agent for TransactionsBalanceGetBalancesByUserIdReturns {
    fn topic() -> &'static str {
        "transactions_balance_getBalancesByUserId"
    }
    fn method() -> &'static str {
        "balance_getBalancesByUserId"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
