// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceAdminIncreaseBalanceParamsExtraDetailParams {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceAdminIncreaseBalanceParams {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "extraDetail")]
    pub extra_detail: Option<TransactionsBalanceAdminIncreaseBalanceParamsExtraDetailParams>,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "creator")]
    pub creator: String,
}
impl Schema for TransactionsBalanceAdminIncreaseBalanceParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"},\"txId\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"},\"extraDetail\":{\"type\":\"object\",\"properties\":{}},\"userId\":{\"type\":\"string\"},\"reason\":{\"type\":\"string\"},\"creator\":{\"type\":\"string\"}},\"required\":[\"creator\",\"reason\",\"currency\",\"amount\",\"txId\",\"userId\"]}") . unwrap ()
    }
}
impl Agent for TransactionsBalanceAdminIncreaseBalanceParams {
    fn topic() -> &'static str {
        "transactions_balance_adminIncreaseBalance"
    }
    fn method() -> &'static str {
        "balance_adminIncreaseBalance"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceAdminIncreaseBalanceReturns {}
impl Schema for TransactionsBalanceAdminIncreaseBalanceReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"object\",\"properties\":{}}")
    }
}
impl Agent for TransactionsBalanceAdminIncreaseBalanceReturns {
    fn topic() -> &'static str {
        "transactions_balance_adminIncreaseBalance"
    }
    fn method() -> &'static str {
        "balance_adminIncreaseBalance"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
