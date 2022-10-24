// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItemsDeleteParams {
    #[serde(rename = "hashItemId")]
    pub hash_item_id: f64,
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
    #[serde(rename = "hashItemId")]
    pub hash_item_id: f64,
}
impl Schema for RefundItemsDeleteParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"hashItemId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"hashItemId\"]}") . unwrap ()
    }
}
impl Agent for RefundItemsDeleteParams {
    fn topic() -> &'static str {
        "refund_items_delete"
    }
    fn method() -> &'static str {
        "items_delete"
    }
    fn agent() -> &'static str {
        "refund"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItemsDeleteReturns(pub bool);
impl Schema for RefundItemsDeleteReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for RefundItemsDeleteReturns {
    fn topic() -> &'static str {
        "refund_items_delete"
    }
    fn method() -> &'static str {
        "items_delete"
    }
    fn agent() -> &'static str {
        "refund"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
