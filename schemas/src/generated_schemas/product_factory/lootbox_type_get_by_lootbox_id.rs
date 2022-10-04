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
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"lootboxId\":{\"type\":\"string\"}},\"required\":[\"lootboxId\"]}") . unwrap ()
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
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "lootboxId")]
    pub lootbox_id: String,
    #[serde(rename = "dropChanceMultiplier")]
    pub drop_chance_multiplier: i32,
    #[serde(rename = "name")]
    pub name: String,
}
impl Schema for ProductFactoryLootboxTypeGetByLootboxIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"lootboxId\":{\"type\":\"string\"},\"dropChanceMultiplier\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"name\":{\"type\":\"string\"}},\"required\":[\"id\",\"name\",\"lootboxId\",\"dropChanceMultiplier\"]}")
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
