// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceAdminIncreaseBalanceParamsExtraDetailsParams {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceAdminIncreaseBalanceParams {
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "extraDetails")]
    pub extra_details: Option<TransactionsBalanceAdminIncreaseBalanceParamsExtraDetailsParams>,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "currency")]
    pub currency: String,
}
impl Schema for TransactionsBalanceAdminIncreaseBalanceParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"creator\":{\"type\":\"string\"},\"extraDetails\":{\"type\":\"object\",\"properties\":{},\"required\":null},\"reason\":{\"type\":\"string\"},\"txId\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)$\"}},\"required\":[\"creator\",\"reason\",\"currency\",\"amount\",\"txId\",\"userId\"]}")
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
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceAdminIncreaseBalanceReturns {}
impl Schema for TransactionsBalanceAdminIncreaseBalanceReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"object\",\"properties\":{},\"required\":null}")
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
}
