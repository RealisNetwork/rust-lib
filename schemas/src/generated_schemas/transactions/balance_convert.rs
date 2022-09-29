// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceConvertParams {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "fromCurrency")]
    pub from_currency: String,
    #[serde(rename = "toCurrency")]
    pub to_currency: String,
}
impl Schema for TransactionsBalanceConvertParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"amount\":{\"type\":\"string\"},\"fromCurrency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"},\"toCurrency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"}},\"required\":[\"fromCurrency\",\"toCurrency\",\"amount\"]}") . unwrap ()
    }
}
impl Agent for TransactionsBalanceConvertParams {
    fn topic() -> &'static str {
        "transactions_balance_convert"
    }
    fn method() -> &'static str {
        "balance_convert"
    }
    fn agent() -> &'static str {
        "transactions"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceConvertReturns {
    #[serde(rename = "isBalanceConverted")]
    pub is_balance_converted: bool,
}
impl Schema for TransactionsBalanceConvertReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"isBalanceConverted\":{\"type\":\"boolean\"}},\"required\":[\"isBalanceConverted\"]}")
    }
}
impl Agent for TransactionsBalanceConvertReturns {
    fn topic() -> &'static str {
        "transactions_balance_convert"
    }
    fn method() -> &'static str {
        "balance_convert"
    }
    fn agent() -> &'static str {
        "transactions"
    }
}
