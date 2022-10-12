// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceAdminDecreaseBalanceParamsExtraDetailParams {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceAdminDecreaseBalanceParams {
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "extraDetail")]
    pub extra_detail: Option<TransactionsBalanceAdminDecreaseBalanceParamsExtraDetailParams>,
    #[serde(rename = "creator")]
    pub creator: String,
}
impl Schema for TransactionsBalanceAdminDecreaseBalanceParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"reason\":{\"type\":\"string\"},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"},\"txId\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"},\"extraDetail\":{\"type\":\"object\",\"properties\":{}},\"creator\":{\"type\":\"string\"}},\"required\":[\"creator\",\"reason\",\"currency\",\"amount\",\"txId\",\"userId\"]}") . unwrap ()
    }
}
impl Agent for TransactionsBalanceAdminDecreaseBalanceParams {
    fn topic() -> &'static str {
        "transactions_balance_adminDecreaseBalance"
    }
    fn method() -> &'static str {
        "balance_adminDecreaseBalance"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceAdminDecreaseBalanceReturns {}
impl Schema for TransactionsBalanceAdminDecreaseBalanceReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"object\",\"properties\":{}}")
    }
}
impl Agent for TransactionsBalanceAdminDecreaseBalanceReturns {
    fn topic() -> &'static str {
        "transactions_balance_adminDecreaseBalance"
    }
    fn method() -> &'static str {
        "balance_adminDecreaseBalance"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
