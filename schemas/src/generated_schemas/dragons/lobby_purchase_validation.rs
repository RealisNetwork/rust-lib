// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct DragonsLobbyPurchaseValidationParams {
    #[serde(rename = "purchaseToken")]
    pub purchase_token: String,
    #[serde(rename = "storeId")]
    pub store_id: i64,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "productId")]
    pub product_id: String,
}
