// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootboxesLootboxesOpenLootboxParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "lootboxId")]
    pub lootbox_id: f64,
}
impl Schema for LootboxesLootboxesOpenLootboxParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"lootboxId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"lootboxId\"]}")
    }
}
impl Agent for LootboxesLootboxesOpenLootboxParams {
    fn topic() -> &'static str {
        "lootboxes_lootboxes_openLootbox"
    }
    fn method() -> &'static str {
        "lootboxes_openLootbox"
    }
    fn agent() -> &'static str {
        "lootboxes"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootboxesLootboxesOpenLootboxReturns {
    #[serde(rename = "BingoItemRewards")]
    pub bingo_item_rewards: Vec<()>,
    #[serde(rename = "ScienceReward")]
    pub science_reward: f64,
    #[serde(rename = "LootboxId")]
    pub lootbox_id: f64,
    #[serde(rename = "JackpotPool")]
    pub jackpot_pool: String,
    #[serde(rename = "CardRewards")]
    pub card_rewards: Vec<()>,
    #[serde(rename = "HardReward")]
    pub hard_reward: f64,
    #[serde(rename = "Status")]
    pub status: f64,
    #[serde(rename = "JackpotReward")]
    pub jackpot_reward: String,
}
impl Schema for LootboxesLootboxesOpenLootboxReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"BingoItemRewards\":{\"type\":\"array\",\"items\":{}},\"ScienceReward\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"LootboxId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"JackpotPool\":{\"type\":\"string\"},\"CardRewards\":{\"type\":\"array\",\"items\":{}},\"HardReward\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"Status\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"JackpotReward\":{\"type\":\"string\"}},\"required\":[\"LootboxId\",\"CardRewards\",\"ScienceReward\",\"HardReward\",\"JackpotReward\",\"JackpotPool\",\"BingoItemRewards\",\"Status\"]}")
    }
}
impl Agent for LootboxesLootboxesOpenLootboxReturns {
    fn topic() -> &'static str {
        "lootboxes_lootboxes_openLootbox"
    }
    fn method() -> &'static str {
        "lootboxes_openLootbox"
    }
    fn agent() -> &'static str {
        "lootboxes"
    }
}
