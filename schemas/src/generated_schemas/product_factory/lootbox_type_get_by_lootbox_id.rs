// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryLootboxTypeGetByLootboxIdParams {
    #[serde(rename = "lootboxId")]
    pub lootbox_id: String,
}
impl Schema for ProductFactoryLootboxTypeGetByLootboxIdParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"lootboxId\":{\"type\":\"string\"}},\"required\":[\"lootboxId\"]}")
    }
}
impl Agent for ProductFactoryLootboxTypeGetByLootboxIdParams {
    fn topic() -> &'static str {
        "productFactory_lootboxType_getByLootboxId"
    }
    fn method() -> &'static str {
        "lootboxType_getByLootboxId"
    }
    fn agent() -> &'static str {
        "productFactory"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryLootboxTypeGetByLootboxIdReturns {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "lootboxId")]
    pub lootbox_id: String,
    #[serde(rename = "dropChanceMultiplier")]
    pub drop_chance_multiplier: i32,
    #[serde(rename = "id")]
    pub id: i32,
}
impl Schema for ProductFactoryLootboxTypeGetByLootboxIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"name\":{\"type\":\"string\"},\"lootboxId\":{\"type\":\"string\"},\"dropChanceMultiplier\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"id\",\"name\",\"lootboxId\",\"dropChanceMultiplier\"]}")
    }
}
impl Agent for ProductFactoryLootboxTypeGetByLootboxIdReturns {
    fn topic() -> &'static str {
        "productFactory_lootboxType_getByLootboxId"
    }
    fn method() -> &'static str {
        "lootboxType_getByLootboxId"
    }
    fn agent() -> &'static str {
        "productFactory"
    }
}
