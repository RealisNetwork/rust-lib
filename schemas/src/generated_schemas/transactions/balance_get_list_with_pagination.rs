// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetListWithPaginationParams {
    #[serde(rename = "page")]
    pub page: i32,
}
impl Schema for TransactionsBalanceGetListWithPaginationParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"page\":{\"type\":\"integer\",\"minimum\":1,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"page\"]}") . unwrap ()
    }
}
impl Agent for TransactionsBalanceGetListWithPaginationParams {
    fn topic() -> &'static str {
        "transactions_balance_getListWithPagination"
    }
    fn method() -> &'static str {
        "balance_getListWithPagination"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetListWithPaginationReturnsListParamsParams {
    #[serde(rename = "balanceChange")]
    pub balance_change: String,
    #[serde(rename = "dateTime")]
    pub date_time: String,
    #[serde(rename = "blockId")]
    pub block_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetListWithPaginationReturns {
    #[serde(rename = "list")]
    pub list: Vec<TransactionsBalanceGetListWithPaginationReturnsListParamsParams>,
    #[serde(rename = "page")]
    pub page: i32,
    #[serde(rename = "pages")]
    pub pages: i32,
}
impl Schema for TransactionsBalanceGetListWithPaginationReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"list\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"balanceChange\":{\"type\":\"string\"},\"dateTime\":{\"type\":\"string\"},\"blockId\":{\"type\":\"string\"}},\"required\":[\"blockId\",\"dateTime\",\"balanceChange\"]}},\"page\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"pages\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"page\",\"pages\",\"list\"]}")
    }
}
impl Agent for TransactionsBalanceGetListWithPaginationReturns {
    fn topic() -> &'static str {
        "transactions_balance_getListWithPagination"
    }
    fn method() -> &'static str {
        "balance_getListWithPagination"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
