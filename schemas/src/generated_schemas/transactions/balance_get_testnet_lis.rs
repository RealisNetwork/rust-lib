// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for TransactionsBalanceGetTestnetLisParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(TransactionsBalanceGetTestnetLisParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct TransactionsBalanceGetTestnetLisParams;
impl Schema for TransactionsBalanceGetTestnetLisParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for TransactionsBalanceGetTestnetLisParams {
    fn topic() -> &'static str {
        "transactions_balance_getTestnetLis"
    }
    fn method() -> &'static str {
        "balance_getTestnetLis"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetTestnetLisReturns(pub bool);
impl Schema for TransactionsBalanceGetTestnetLisReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for TransactionsBalanceGetTestnetLisReturns {
    fn topic() -> &'static str {
        "transactions_balance_getTestnetLis"
    }
    fn method() -> &'static str {
        "balance_getTestnetLis"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
