// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootboxesLootboxesGetLootboxesInfoParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for LootboxesLootboxesGetLootboxesInfoParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
impl Agent for LootboxesLootboxesGetLootboxesInfoParams {
    fn topic() -> &'static str {
        "lootboxes_lootboxes_getLootboxesInfo"
    }
    fn method() -> &'static str {
        "lootboxes_getLootboxesInfo"
    }
    fn agent() -> &'static str {
        "lootboxes"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootboxesLootboxesGetLootboxesInfoReturns {
    #[serde(rename = "UsualLootboxes")]
    pub usual_lootboxes: Vec<f64>,
    #[serde(rename = "CooldownedLootboxes")]
    pub cooldowned_lootboxes: Vec<f64>,
    #[serde(rename = "Status")]
    pub status: f64,
}
impl Schema for LootboxesLootboxesGetLootboxesInfoReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"UsualLootboxes\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"CooldownedLootboxes\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"Status\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"CooldownedLootboxes\",\"UsualLootboxes\",\"Status\"]}")
    }
}
impl Agent for LootboxesLootboxesGetLootboxesInfoReturns {
    fn topic() -> &'static str {
        "lootboxes_lootboxes_getLootboxesInfo"
    }
    fn method() -> &'static str {
        "lootboxes_getLootboxesInfo"
    }
    fn agent() -> &'static str {
        "lootboxes"
    }
}
