// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct RefundItemsGetAllParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RefundItemsGetAllReturnsParams {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "hashItemId")]
    pub hash_item_id: i64,
    #[serde(rename = "lockedUntil")]
    pub locked_until: i64,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "userId")]
    pub user_id: String,
}
pub type RefundItemsGetAllReturns = Vec<RefundItemsGetAllReturnsParams>;
