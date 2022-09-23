// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyBuyUsualLootboxParams {
    #[serde(rename = "lootboxId")]
    pub lootbox_id: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsLobbyBuyUsualLootboxParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"lootboxId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"lootboxId\"]}")
    }
}
impl Agent for CatsLobbyBuyUsualLootboxParams {
    fn topic() -> &'static str {
        "cats_lobby_buyUsualLootbox"
    }
    fn method() -> &'static str {
        "lobby_buyUsualLootbox"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
impl<'de> Deserialize<'de> for CatsLobbyBuyUsualLootboxReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyBuyUsualLootboxReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyBuyUsualLootboxReturns;
impl Schema for CatsLobbyBuyUsualLootboxReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyBuyUsualLootboxReturns {
    fn topic() -> &'static str {
        "cats_lobby_buyUsualLootbox"
    }
    fn method() -> &'static str {
        "lobby_buyUsualLootbox"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
