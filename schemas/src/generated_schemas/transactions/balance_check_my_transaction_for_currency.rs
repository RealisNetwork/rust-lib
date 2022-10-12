// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceCheckMyTransactionForCurrencyParams {
    #[serde(rename = "currencies")]
    pub currencies: Vec<String>,
}
impl Schema for TransactionsBalanceCheckMyTransactionForCurrencyParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"currencies\":{\"type\":\"array\",\"items\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"}}},\"required\":[\"currencies\"]}") . unwrap ()
    }
}
impl Agent for TransactionsBalanceCheckMyTransactionForCurrencyParams {
    fn topic() -> &'static str {
        "transactions_balance_checkMyTransactionForCurrency"
    }
    fn method() -> &'static str {
        "balance_checkMyTransactionForCurrency"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceCheckMyTransactionForCurrencyReturns(pub bool);
impl Schema for TransactionsBalanceCheckMyTransactionForCurrencyReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for TransactionsBalanceCheckMyTransactionForCurrencyReturns {
    fn topic() -> &'static str {
        "transactions_balance_checkMyTransactionForCurrency"
    }
    fn method() -> &'static str {
        "balance_checkMyTransactionForCurrency"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
