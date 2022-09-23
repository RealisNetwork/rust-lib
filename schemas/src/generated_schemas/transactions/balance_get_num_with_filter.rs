// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetNumWithFilterParams {
    #[serde(rename = "debit")]
    pub debit: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    #[serde(rename = "credit")]
    pub credit: Option<String>,
}
impl Schema for TransactionsBalanceGetNumWithFilterParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"debit\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"credit\":{\"type\":\"string\"}},\"required\":null}")
    }
}
impl Agent for TransactionsBalanceGetNumWithFilterParams {
    fn topic() -> &'static str {
        "transactions_balance_getNumWithFilter"
    }
    fn method() -> &'static str {
        "balance_getNumWithFilter"
    }
    fn agent() -> &'static str {
        "transactions"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetNumWithFilterReturns(pub f64);
impl Schema for TransactionsBalanceGetNumWithFilterReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}")
    }
}
impl Agent for TransactionsBalanceGetNumWithFilterReturns {
    fn topic() -> &'static str {
        "transactions_balance_getNumWithFilter"
    }
    fn method() -> &'static str {
        "balance_getNumWithFilter"
    }
    fn agent() -> &'static str {
        "transactions"
    }
}
