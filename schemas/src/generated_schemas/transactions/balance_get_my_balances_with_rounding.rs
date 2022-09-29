// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for TransactionsBalanceGetMyBalancesWithRoundingParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(TransactionsBalanceGetMyBalancesWithRoundingParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct TransactionsBalanceGetMyBalancesWithRoundingParams;
impl Schema for TransactionsBalanceGetMyBalancesWithRoundingParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for TransactionsBalanceGetMyBalancesWithRoundingParams {
    fn topic() -> &'static str {
        "transactions_balance_getMyBalancesWithRounding"
    }
    fn method() -> &'static str {
        "balance_getMyBalancesWithRounding"
    }
    fn agent() -> &'static str {
        "transactions"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetMyBalancesWithRoundingReturnsTickersParamsParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "amount")]
    pub amount: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetMyBalancesWithRoundingReturns {
    #[serde(rename = "tickers")]
    pub tickers: Vec<TransactionsBalanceGetMyBalancesWithRoundingReturnsTickersParamsParams>,
}
impl Schema for TransactionsBalanceGetMyBalancesWithRoundingReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"tickers\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"ticker\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"}},\"required\":[\"ticker\",\"amount\"]}}},\"required\":[\"tickers\"]}")
    }
}
impl Agent for TransactionsBalanceGetMyBalancesWithRoundingReturns {
    fn topic() -> &'static str {
        "transactions_balance_getMyBalancesWithRounding"
    }
    fn method() -> &'static str {
        "balance_getMyBalancesWithRounding"
    }
    fn agent() -> &'static str {
        "transactions"
    }
}
