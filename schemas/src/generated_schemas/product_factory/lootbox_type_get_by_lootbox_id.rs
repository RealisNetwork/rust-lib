// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryLootboxTypeGetByLootboxIdParams {
    #[serde(rename = "lootboxId")]
    pub lootbox_id: String,
}
impl Schema for ProductFactoryLootboxTypeGetByLootboxIdParams {
    fn schema() -> Value {
        todo!()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryLootboxTypeGetByLootboxIdReturns {
    #[serde(rename = "lootboxId")]
    pub lootbox_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "dropChanceMultiplier")]
    pub drop_chance_multiplier: i32,
}
