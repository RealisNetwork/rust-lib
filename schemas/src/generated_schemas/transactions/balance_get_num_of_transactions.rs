// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetNumOfTransactionsParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for TransactionsBalanceGetNumOfTransactionsParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for TransactionsBalanceGetNumOfTransactionsParams {
    fn topic() -> &'static str {
        "transactions_balance_getNumOfTransactions"
    }
    fn method() -> &'static str {
        "balance_getNumOfTransactions"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetNumOfTransactionsReturns(pub i32);
impl Schema for TransactionsBalanceGetNumOfTransactionsReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}")
    }
}
impl Agent for TransactionsBalanceGetNumOfTransactionsReturns {
    fn topic() -> &'static str {
        "transactions_balance_getNumOfTransactions"
    }
    fn method() -> &'static str {
        "balance_getNumOfTransactions"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
