// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItemsAddParams {
    #[serde(rename = "hashItemId")]
    pub hash_item_id: i64,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for RefundItemsAddParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"hashItemId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"hashItemId\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItemsAddReturns(bool);
impl Schema for RefundItemsAddReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
