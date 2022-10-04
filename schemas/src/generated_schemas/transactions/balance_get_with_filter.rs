// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetWithFilterParamsFilterListParams {
    #[serde(rename = "firstDate")]
    pub first_date: Option<String>,
    #[serde(rename = "lastDate")]
    pub last_date: Option<String>,
    #[serde(rename = "reason")]
    pub reason: Option<String>,
    #[serde(rename = "TypeTransaction")]
    pub type_transaction: Option<String>,
    #[serde(rename = "creator")]
    pub creator: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    #[serde(rename = "page")]
    pub page: Option<f64>,
    #[serde(rename = "perPage")]
    pub per_page: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetWithFilterParams {
    #[serde(rename = "filterList")]
    pub filter_list: TransactionsBalanceGetWithFilterParamsFilterListParams,
}
impl Schema for TransactionsBalanceGetWithFilterParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"filterList\":{\"type\":\"object\",\"properties\":{\"firstDate\":{\"type\":\"string\"},\"lastDate\":{\"type\":\"string\"},\"reason\":{\"type\":\"string\"},\"TypeTransaction\":{\"type\":\"string\"},\"creator\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}}}},\"required\":[\"filterList\"]}") . unwrap ()
    }
}
impl Agent for TransactionsBalanceGetWithFilterParams {
    fn topic() -> &'static str {
        "transactions_balance_getWithFilter"
    }
    fn method() -> &'static str {
        "balance_getWithFilter"
    }
    fn agent() -> &'static str {
        "transactions"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetWithFilterReturnsDataParamsParamsExtraDetailParams {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetWithFilterReturnsDataParamsParams {
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "debit")]
    pub debit: String,
    #[serde(rename = "credit")]
    pub credit: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "extraDetail")]
    pub extra_detail:
        Option<TransactionsBalanceGetWithFilterReturnsDataParamsParamsExtraDetailParams>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetWithFilterReturns {
    #[serde(rename = "data")]
    pub data: Vec<TransactionsBalanceGetWithFilterReturnsDataParamsParams>,
    #[serde(rename = "totalCount")]
    pub total_count: f64,
}
impl Schema for TransactionsBalanceGetWithFilterReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"data\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"creator\":{\"type\":\"string\"},\"debit\":{\"type\":\"string\"},\"credit\":{\"type\":\"string\"},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"},\"txId\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"reason\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"extraDetail\":{\"type\":\"object\",\"properties\":{}}},\"required\":[\"id\",\"debit\",\"credit\",\"reason\",\"currency\",\"txId\",\"userId\",\"creator\",\"createdAt\",\"updatedAt\"]}},\"totalCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"totalCount\",\"data\"]}")
    }
}
impl Agent for TransactionsBalanceGetWithFilterReturns {
    fn topic() -> &'static str {
        "transactions_balance_getWithFilter"
    }
    fn method() -> &'static str {
        "balance_getWithFilter"
    }
    fn agent() -> &'static str {
        "transactions"
    }
}
