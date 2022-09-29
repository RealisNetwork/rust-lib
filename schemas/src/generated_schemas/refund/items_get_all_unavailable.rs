// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItemsGetAllUnavailableParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for RefundItemsGetAllUnavailableParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for RefundItemsGetAllUnavailableParams {
    fn topic() -> &'static str {
        "refund_items_getAllUnavailable"
    }
    fn method() -> &'static str {
        "items_getAllUnavailable"
    }
    fn agent() -> &'static str {
        "refund"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItemsGetAllUnavailableReturnsParams {
    #[serde(rename = "lockedUntil")]
    pub locked_until: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "hashItemId")]
    pub hash_item_id: f64,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItemsGetAllUnavailableReturns(pub Vec<RefundItemsGetAllUnavailableReturnsParams>);
impl Schema for RefundItemsGetAllUnavailableReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"lockedUntil\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"hashItemId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"updatedAt\":{\"type\":\"string\"}},\"required\":[\"id\",\"userId\",\"hashItemId\",\"lockedUntil\",\"createdAt\",\"updatedAt\"]}}")
    }
}
impl Agent for RefundItemsGetAllUnavailableReturns {
    fn topic() -> &'static str {
        "refund_items_getAllUnavailable"
    }
    fn method() -> &'static str {
        "items_getAllUnavailable"
    }
    fn agent() -> &'static str {
        "refund"
    }
}
