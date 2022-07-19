// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItemsDeleteParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "hashItemId")]
    pub hash_item_id: i64,
}
impl Schema for RefundItemsDeleteParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"hashItemId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"hashItemId\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItemsDeleteReturns(bool);
impl Schema for RefundItemsDeleteReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
