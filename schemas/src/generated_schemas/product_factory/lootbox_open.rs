// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductFactoryLootboxOpenParams {
    #[serde(rename = "lootboxBindingId")]
    pub lootbox_binding_id: i64,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "excludedPersonalTypes")]
    pub excluded_personal_types: Vec<String>,
    #[serde(rename = "productId")]
    pub product_id: i64,
}
pub type ProductFactoryLootboxOpenReturns = bool;
