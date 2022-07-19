// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryLootboxOpenParams {
    #[serde(rename = "excludedPersonalTypes")]
    pub excluded_personal_types: Vec<String>,
    #[serde(rename = "productId")]
    pub product_id: i64,
    #[serde(rename = "lootboxBindingId")]
    pub lootbox_binding_id: i64,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for ProductFactoryLootboxOpenParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"excludedPersonalTypes\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"productId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"lootboxBindingId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"productId\",\"userId\",\"lootboxBindingId\",\"excludedPersonalTypes\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryLootboxOpenReturns(bool);
impl Schema for ProductFactoryLootboxOpenReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}