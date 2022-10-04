// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for RefundItemsGetAllMyParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(RefundItemsGetAllMyParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct RefundItemsGetAllMyParams;
impl Schema for RefundItemsGetAllMyParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for RefundItemsGetAllMyParams {
    fn topic() -> &'static str {
        "refund_items_getAllMy"
    }
    fn method() -> &'static str {
        "items_getAllMy"
    }
    fn agent() -> &'static str {
        "refund"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItemsGetAllMyReturnsParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "lockedUntil")]
    pub locked_until: f64,
    #[serde(rename = "hashItemId")]
    pub hash_item_id: f64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "id")]
    pub id: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItemsGetAllMyReturns(pub Vec<RefundItemsGetAllMyReturnsParams>);
impl Schema for RefundItemsGetAllMyReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"lockedUntil\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"hashItemId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"createdAt\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"userId\",\"hashItemId\",\"lockedUntil\",\"createdAt\",\"updatedAt\"]}}")
    }
}
impl Agent for RefundItemsGetAllMyReturns {
    fn topic() -> &'static str {
        "refund_items_getAllMy"
    }
    fn method() -> &'static str {
        "items_getAllMy"
    }
    fn agent() -> &'static str {
        "refund"
    }
}
