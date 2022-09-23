// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetAllParams {
    #[serde(rename = "perPage")]
    pub per_page: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "page")]
    pub page: f64,
}
impl Schema for TransactionsBalanceGetAllParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"},\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"page\",\"perPage\"]}")
    }
}
impl Agent for TransactionsBalanceGetAllParams {
    fn topic() -> &'static str {
        "transactions_balance_getAll"
    }
    fn method() -> &'static str {
        "balance_getAll"
    }
    fn agent() -> &'static str {
        "transactions"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetAllReturnsDataParamsParamsExtraDetailParams {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetAllReturnsDataParamsParams {
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "credit")]
    pub credit: String,
    #[serde(rename = "extraDetail")]
    pub extra_detail: Option<TransactionsBalanceGetAllReturnsDataParamsParamsExtraDetailParams>,
    #[serde(rename = "debit")]
    pub debit: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "reason")]
    pub reason: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetAllReturns {
    #[serde(rename = "totalCount")]
    pub total_count: f64,
    #[serde(rename = "data")]
    pub data: Vec<TransactionsBalanceGetAllReturnsDataParamsParams>,
}
impl Schema for TransactionsBalanceGetAllReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"totalCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"data\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"creator\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"credit\":{\"type\":\"string\"},\"extraDetail\":{\"type\":\"object\",\"properties\":{},\"required\":null},\"debit\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)$\"},\"txId\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"reason\":{\"type\":\"string\"}},\"required\":[\"id\",\"debit\",\"credit\",\"reason\",\"currency\",\"txId\",\"userId\",\"creator\",\"createdAt\",\"updatedAt\"]}}},\"required\":[\"totalCount\",\"data\"]}")
    }
}
impl Agent for TransactionsBalanceGetAllReturns {
    fn topic() -> &'static str {
        "transactions_balance_getAll"
    }
    fn method() -> &'static str {
        "balance_getAll"
    }
    fn agent() -> &'static str {
        "transactions"
    }
}
