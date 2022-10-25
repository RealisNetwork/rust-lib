// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for TransactionsBalanceGetAllCreditTransactionListParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(TransactionsBalanceGetAllCreditTransactionListParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct TransactionsBalanceGetAllCreditTransactionListParams;
impl Schema for TransactionsBalanceGetAllCreditTransactionListParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for TransactionsBalanceGetAllCreditTransactionListParams {
    fn topic() -> &'static str {
        "transactions_balance_getAllCreditTransactionList"
    }
    fn method() -> &'static str {
        "balance_getAllCreditTransactionList"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetAllCreditTransactionListReturnsParams {
    #[serde(rename = "date", deserialize_with = "deserialize_to_string")]
    pub date: String,
    #[serde(rename = "amount", deserialize_with = "deserialize_to_string")]
    pub amount: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetAllCreditTransactionListReturns(
    pub Vec<TransactionsBalanceGetAllCreditTransactionListReturnsParams>,
);
impl Schema for TransactionsBalanceGetAllCreditTransactionListReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"date\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"}},\"required\":[\"amount\",\"date\"]}}")
    }
}
impl Agent for TransactionsBalanceGetAllCreditTransactionListReturns {
    fn topic() -> &'static str {
        "transactions_balance_getAllCreditTransactionList"
    }
    fn method() -> &'static str {
        "balance_getAllCreditTransactionList"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
