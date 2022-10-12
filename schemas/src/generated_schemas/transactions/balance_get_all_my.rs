// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetAllMyParams {
    #[serde(rename = "page")]
    pub page: i32,
    #[serde(rename = "perPage")]
    pub per_page: i32,
}
impl Schema for TransactionsBalanceGetAllMyParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"page\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"perPage\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"page\",\"perPage\"]}") . unwrap ()
    }
}
impl Agent for TransactionsBalanceGetAllMyParams {
    fn topic() -> &'static str {
        "transactions_balance_getAllMy"
    }
    fn method() -> &'static str {
        "balance_getAllMy"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetAllMyReturnsParamsExtraDetailParams {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetAllMyReturnsParams {
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "debit")]
    pub debit: String,
    #[serde(rename = "totalCount")]
    pub total_count: i32,
    #[serde(rename = "credit")]
    pub credit: String,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "extraDetail")]
    pub extra_detail: Option<TransactionsBalanceGetAllMyReturnsParamsExtraDetailParams>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "txId")]
    pub tx_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetAllMyReturns(pub Vec<TransactionsBalanceGetAllMyReturnsParams>);
impl Schema for TransactionsBalanceGetAllMyReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"updatedAt\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"},\"creator\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"debit\":{\"type\":\"string\"},\"totalCount\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"credit\":{\"type\":\"string\"},\"reason\":{\"type\":\"string\"},\"extraDetail\":{\"type\":\"object\",\"properties\":{}},\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"txId\":{\"type\":\"string\"}},\"required\":[\"totalCount\",\"id\",\"debit\",\"credit\",\"reason\",\"currency\",\"txId\",\"userId\",\"creator\",\"createdAt\",\"updatedAt\"]}}")
    }
}
impl Agent for TransactionsBalanceGetAllMyReturns {
    fn topic() -> &'static str {
        "transactions_balance_getAllMy"
    }
    fn method() -> &'static str {
        "balance_getAllMy"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
