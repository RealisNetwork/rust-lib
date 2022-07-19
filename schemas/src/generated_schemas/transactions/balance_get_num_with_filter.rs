// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetNumWithFilterParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "credit")]
    pub credit: String,
    #[serde(rename = "debit")]
    pub debit: String,
}
impl Schema for TransactionsBalanceGetNumWithFilterParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"credit\":{\"type\":\"string\"},\"debit\":{\"type\":\"string\"}},\"required\":null}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetNumWithFilterReturns(i64);
impl Schema for TransactionsBalanceGetNumWithFilterReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}")
    }
}