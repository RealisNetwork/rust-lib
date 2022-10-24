// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyBuyUsualLootboxParams {
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
    #[serde(rename = "lootboxId")]
    pub lootbox_id: f64,
}
impl Schema for DragonsLobbyBuyUsualLootboxParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"lootboxId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"lootboxId\"]}") . unwrap ()
    }
}
impl Agent for DragonsLobbyBuyUsualLootboxParams {
    fn topic() -> &'static str {
        "dragons_lobby_buyUsualLootbox"
    }
    fn method() -> &'static str {
        "lobby_buyUsualLootbox"
    }
    fn agent() -> &'static str {
        "dragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyBuyUsualLootboxReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragonsLobbyBuyUsualLootboxReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyBuyUsualLootboxReturns;
impl Schema for DragonsLobbyBuyUsualLootboxReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragonsLobbyBuyUsualLootboxReturns {
    fn topic() -> &'static str {
        "dragons_lobby_buyUsualLootbox"
    }
    fn method() -> &'static str {
        "lobby_buyUsualLootbox"
    }
    fn agent() -> &'static str {
        "dragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
