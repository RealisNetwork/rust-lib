// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItemsGetAllParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for RefundItemsGetAllParams {
    fn schema() -> Value {
        todo!()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItemsGetAllReturnsParams {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "lockedUntil")]
    pub locked_until: i64,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "hashItemId")]
    pub hash_item_id: i64,
}
pub type RefundItemsGetAllReturns = Vec<RefundItemsGetAllReturnsParams>;
