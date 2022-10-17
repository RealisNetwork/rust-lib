// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItemsGetAllParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for RefundItemsGetAllParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for RefundItemsGetAllParams {
    fn topic() -> &'static str {
        "refund_items_getAll"
    }
    fn method() -> &'static str {
        "items_getAll"
    }
    fn agent() -> &'static str {
        "refund"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItemsGetAllReturnsParams {
    #[serde(rename = "hashItemId")]
    pub hash_item_id: f64,
    #[serde(rename = "lockedUntil")]
    pub locked_until: f64,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItemsGetAllReturns(pub Vec<RefundItemsGetAllReturnsParams>);
impl Schema for RefundItemsGetAllReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"hashItemId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"lockedUntil\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"updatedAt\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"}},\"required\":[\"id\",\"userId\",\"hashItemId\",\"lockedUntil\",\"createdAt\",\"updatedAt\"]}}")
    }
}
impl Agent for RefundItemsGetAllReturns {
    fn topic() -> &'static str {
        "refund_items_getAll"
    }
    fn method() -> &'static str {
        "items_getAll"
    }
    fn agent() -> &'static str {
        "refund"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
