// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootboxesLootboxesOpenLootboxParams {
    #[serde(rename = "lootboxId")]
    pub lootbox_id: f64,
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
}
impl Schema for LootboxesLootboxesOpenLootboxParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"lootboxId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"lootboxId\"]}") . unwrap ()
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
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootboxesLootboxesOpenLootboxReturns {
    #[serde(rename = "Status")]
    pub status: f64,
    #[serde(rename = "JackpotPool", deserialize_with = "deserialize_to_string")]
    pub jackpot_pool: String,
    #[serde(rename = "HardReward")]
    pub hard_reward: f64,
    #[serde(rename = "LootboxId")]
    pub lootbox_id: f64,
    #[serde(rename = "ScienceReward")]
    pub science_reward: f64,
    #[serde(rename = "BingoItemRewards")]
    pub bingo_item_rewards: Vec<()>,
    #[serde(rename = "CardRewards")]
    pub card_rewards: Vec<()>,
    #[serde(rename = "JackpotReward", deserialize_with = "deserialize_to_string")]
    pub jackpot_reward: String,
}
impl Schema for LootboxesLootboxesOpenLootboxReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"Status\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"JackpotPool\":{\"type\":\"string\"},\"HardReward\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"LootboxId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ScienceReward\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"BingoItemRewards\":{\"type\":\"array\",\"items\":{}},\"CardRewards\":{\"type\":\"array\",\"items\":{}},\"JackpotReward\":{\"type\":\"string\"}},\"required\":[\"LootboxId\",\"CardRewards\",\"ScienceReward\",\"HardReward\",\"JackpotReward\",\"JackpotPool\",\"BingoItemRewards\",\"Status\"]}")
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
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
