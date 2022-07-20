// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItemsIsAvailableParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "hashItemId")]
    pub hash_item_id: i64,
}
impl Schema for RefundItemsIsAvailableParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"hashItemId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"hashItemId\"]}")
    }
}
impl Agent for RefundItemsIsAvailableParams {
    fn topic() -> &'static str {
        "refund_items_isAvailable"
    }
    fn method() -> &'static str {
        "items_isAvailable"
    }
    fn agent() -> &'static str {
        "refund"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItemsIsAvailableReturns(bool);
impl Schema for RefundItemsIsAvailableReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for RefundItemsIsAvailableReturns {
    fn topic() -> &'static str {
        "refund_items_isAvailable"
    }
    fn method() -> &'static str {
        "items_isAvailable"
    }
    fn agent() -> &'static str {
        "refund"
    }
}
