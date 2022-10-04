// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsProductFactoryLootboxOpenParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "excludedPersonalTypes")]
    pub excluded_personal_types: Vec<String>,
    #[serde(rename = "productId")]
    pub product_id: f64,
    #[serde(rename = "lootboxBindingId")]
    pub lootbox_binding_id: f64,
}
impl Schema for DragocatsProductFactoryLootboxOpenParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"excludedPersonalTypes\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"productId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"lootboxBindingId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"productId\",\"userId\",\"lootboxBindingId\",\"excludedPersonalTypes\"]}") . unwrap ()
    }
}
impl Agent for DragocatsProductFactoryLootboxOpenParams {
    fn topic() -> &'static str {
        "dragocats-product-factory_lootbox_open"
    }
    fn method() -> &'static str {
        "lootbox_open"
    }
    fn agent() -> &'static str {
        "dragocats-product-factory"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsProductFactoryLootboxOpenReturns(pub bool);
impl Schema for DragocatsProductFactoryLootboxOpenReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for DragocatsProductFactoryLootboxOpenReturns {
    fn topic() -> &'static str {
        "dragocats-product-factory_lootbox_open"
    }
    fn method() -> &'static str {
        "lootbox_open"
    }
    fn agent() -> &'static str {
        "dragocats-product-factory"
    }
}
