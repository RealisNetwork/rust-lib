// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryLootboxTypeUpdateByLootboxIdParams {
    #[serde(rename = "dropChanceMultiplier")]
    pub drop_chance_multiplier: Option<i32>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "lootboxId")]
    pub lootbox_id: i32,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
impl Schema for ProductFactoryLootboxTypeUpdateByLootboxIdParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"dropChanceMultiplier\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"name\":{\"type\":\"string\"},\"lootboxId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"type\":{\"type\":\"string\"}},\"required\":[\"lootboxId\"]}") . unwrap ()
    }
}
impl Agent for ProductFactoryLootboxTypeUpdateByLootboxIdParams {
    fn topic() -> &'static str {
        "productFactory_lootboxType_updateByLootboxId"
    }
    fn method() -> &'static str {
        "lootboxType_updateByLootboxId"
    }
    fn agent() -> &'static str {
        "productFactory"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryLootboxTypeUpdateByLootboxIdReturns(pub bool);
impl Schema for ProductFactoryLootboxTypeUpdateByLootboxIdReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for ProductFactoryLootboxTypeUpdateByLootboxIdReturns {
    fn topic() -> &'static str {
        "productFactory_lootboxType_updateByLootboxId"
    }
    fn method() -> &'static str {
        "lootboxType_updateByLootboxId"
    }
    fn agent() -> &'static str {
        "productFactory"
    }
}
