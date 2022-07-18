// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryLootboxOpenParams {
    #[serde(rename = "productId")]
    pub product_id: i64,
    #[serde(rename = "lootboxBindingId")]
    pub lootbox_binding_id: i64,
    #[serde(rename = "excludedPersonalTypes")]
    pub excluded_personal_types: Vec<String>,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for ProductFactoryLootboxOpenParams {
    fn schema() -> Value {
        todo!()
    }
}
pub type ProductFactoryLootboxOpenReturns = bool;
