// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryLootboxOpenParams {
    #[serde(rename = "lootboxBindingId")]
    pub lootbox_binding_id: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "excludedPersonalTypes")]
    pub excluded_personal_types: Vec<String>,
    #[serde(rename = "productId")]
    pub product_id: f64,
}
impl Schema for ProductFactoryLootboxOpenParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"lootboxBindingId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"},\"excludedPersonalTypes\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"productId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"productId\",\"userId\",\"lootboxBindingId\",\"excludedPersonalTypes\"]}") . unwrap ()
    }
}
impl Agent for ProductFactoryLootboxOpenParams {
    fn topic() -> &'static str {
        "product-factory_lootbox_open"
    }
    fn method() -> &'static str {
        "lootbox_open"
    }
    fn agent() -> &'static str {
        "product-factory"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryLootboxOpenReturns(pub bool);
impl Schema for ProductFactoryLootboxOpenReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for ProductFactoryLootboxOpenReturns {
    fn topic() -> &'static str {
        "product-factory_lootbox_open"
    }
    fn method() -> &'static str {
        "lootbox_open"
    }
    fn agent() -> &'static str {
        "product-factory"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
