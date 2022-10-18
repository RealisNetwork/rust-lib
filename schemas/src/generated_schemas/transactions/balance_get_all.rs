// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetAllParams {
    #[serde(rename = "page")]
    pub page: i32,
    #[serde(rename = "perPage")]
    pub per_page: i32,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for TransactionsBalanceGetAllParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"page\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"perPage\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"page\",\"perPage\"]}") . unwrap ()
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetAllReturnsDataParamsParamsExtraDetailParams {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetAllReturnsDataParamsParams {
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "credit")]
    pub credit: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "debit")]
    pub debit: String,
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "extraDetail")]
    pub extra_detail: Option<TransactionsBalanceGetAllReturnsDataParamsParamsExtraDetailParams>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetAllReturns {
    #[serde(rename = "data")]
    pub data: Vec<TransactionsBalanceGetAllReturnsDataParamsParams>,
    #[serde(rename = "totalCount")]
    pub total_count: f64,
}
impl Schema for TransactionsBalanceGetAllReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"data\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"updatedAt\":{\"type\":\"string\"},\"credit\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"txId\":{\"type\":\"string\"},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"},\"createdAt\":{\"type\":\"string\"},\"reason\":{\"type\":\"string\"},\"debit\":{\"type\":\"string\"},\"creator\":{\"type\":\"string\"},\"extraDetail\":{\"type\":\"object\",\"properties\":{}}},\"required\":[\"id\",\"debit\",\"credit\",\"reason\",\"currency\",\"txId\",\"userId\",\"creator\",\"createdAt\",\"updatedAt\"]}},\"totalCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"totalCount\",\"data\"]}")
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
