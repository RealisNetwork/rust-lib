// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootboxesLootboxesGetUsualLootboxTimeParams {}
impl Schema for LootboxesLootboxesGetUsualLootboxTimeParams {
    fn schema() -> Value {
        serde_json::from_str("{\"type\":\"object\",\"properties\":{}}").unwrap()
    }
}
impl Agent for LootboxesLootboxesGetUsualLootboxTimeParams {
    fn topic() -> &'static str {
        "lootboxes_lootboxes_getUsualLootboxTime"
    }
    fn method() -> &'static str {
        "lootboxes_getUsualLootboxTime"
    }
    fn agent() -> &'static str {
        "lootboxes"
    }
}
impl<'de> Deserialize<'de> for LootboxesLootboxesGetUsualLootboxTimeReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(LootboxesLootboxesGetUsualLootboxTimeReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct LootboxesLootboxesGetUsualLootboxTimeReturns;
impl Schema for LootboxesLootboxesGetUsualLootboxTimeReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for LootboxesLootboxesGetUsualLootboxTimeReturns {
    fn topic() -> &'static str {
        "lootboxes_lootboxes_getUsualLootboxTime"
    }
    fn method() -> &'static str {
        "lootboxes_getUsualLootboxTime"
    }
    fn agent() -> &'static str {
        "lootboxes"
    }
}
