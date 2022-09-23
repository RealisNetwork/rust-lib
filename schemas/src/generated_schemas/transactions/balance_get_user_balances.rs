// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetUserBalancesParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for TransactionsBalanceGetUserBalancesParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
impl Agent for TransactionsBalanceGetUserBalancesParams {
    fn topic() -> &'static str {
        "transactions_balance_getUserBalances"
    }
    fn method() -> &'static str {
        "balance_getUserBalances"
    }
    fn agent() -> &'static str {
        "transactions"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetUserBalancesReturns {
    #[serde(rename = "WLIS")]
    pub wlis: String,
    #[serde(rename = "ETH")]
    pub eth: String,
    #[serde(rename = "LIS")]
    pub lis: String,
}
impl Schema for TransactionsBalanceGetUserBalancesReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"WLIS\":{\"type\":\"string\"},\"ETH\":{\"type\":\"string\"},\"LIS\":{\"type\":\"string\"}},\"required\":[\"ETH\",\"LIS\",\"WLIS\"]}")
    }
}
impl Agent for TransactionsBalanceGetUserBalancesReturns {
    fn topic() -> &'static str {
        "transactions_balance_getUserBalances"
    }
    fn method() -> &'static str {
        "balance_getUserBalances"
    }
    fn agent() -> &'static str {
        "transactions"
    }
}
