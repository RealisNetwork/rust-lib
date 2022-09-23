// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceDecreaseBalanceByUserIdParamsExtraDetailParams {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceDecreaseBalanceByUserIdParams {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "extraDetail")]
    pub extra_detail: Option<TransactionsBalanceDecreaseBalanceByUserIdParamsExtraDetailParams>,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "reason")]
    pub reason: String,
}
impl Schema for TransactionsBalanceDecreaseBalanceByUserIdParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"amount\":{\"type\":\"string\"},\"creator\":{\"type\":\"string\"},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)$\"},\"userId\":{\"type\":\"string\"},\"extraDetail\":{\"type\":\"object\",\"properties\":{},\"required\":null},\"txId\":{\"type\":\"string\"},\"reason\":{\"type\":\"string\"}},\"required\":[\"creator\",\"reason\",\"currency\",\"amount\",\"txId\",\"userId\"]}")
    }
}
impl Agent for TransactionsBalanceDecreaseBalanceByUserIdParams {
    fn topic() -> &'static str {
        "transactions_balance_decreaseBalanceByUserId"
    }
    fn method() -> &'static str {
        "balance_decreaseBalanceByUserId"
    }
    fn agent() -> &'static str {
        "transactions"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceDecreaseBalanceByUserIdReturns {}
impl Schema for TransactionsBalanceDecreaseBalanceByUserIdReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"object\",\"properties\":{},\"required\":null}")
    }
}
impl Agent for TransactionsBalanceDecreaseBalanceByUserIdReturns {
    fn topic() -> &'static str {
        "transactions_balance_decreaseBalanceByUserId"
    }
    fn method() -> &'static str {
        "balance_decreaseBalanceByUserId"
    }
    fn agent() -> &'static str {
        "transactions"
    }
}
