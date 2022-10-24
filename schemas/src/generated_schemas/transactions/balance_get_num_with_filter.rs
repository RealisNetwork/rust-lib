// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetNumWithFilterParams {
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    #[serde(rename = "credit")]
    pub credit: Option<String>,
    #[serde(rename = "debit")]
    pub debit: Option<String>,
}
impl Schema for TransactionsBalanceGetNumWithFilterParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"credit\":{\"type\":\"string\"},\"debit\":{\"type\":\"string\"}}}") . unwrap ()
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetNumWithFilterReturns(pub i32);
impl Schema for TransactionsBalanceGetNumWithFilterReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}")
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
