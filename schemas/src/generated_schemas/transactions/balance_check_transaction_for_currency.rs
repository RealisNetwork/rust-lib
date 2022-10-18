// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceCheckTransactionForCurrencyParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "currencies")]
    pub currencies: Vec<String>,
}
impl Schema for TransactionsBalanceCheckTransactionForCurrencyParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"currencies\":{\"type\":\"array\",\"items\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"}}},\"required\":[\"userId\",\"currencies\"]}") . unwrap ()
    }
}
impl Agent for TransactionsBalanceCheckTransactionForCurrencyParams {
    fn topic() -> &'static str {
        "transactions_balance_checkTransactionForCurrency"
    }
    fn method() -> &'static str {
        "balance_checkTransactionForCurrency"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceCheckTransactionForCurrencyReturns(pub bool);
impl Schema for TransactionsBalanceCheckTransactionForCurrencyReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for TransactionsBalanceCheckTransactionForCurrencyReturns {
    fn topic() -> &'static str {
        "transactions_balance_checkTransactionForCurrency"
    }
    fn method() -> &'static str {
        "balance_checkTransactionForCurrency"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
