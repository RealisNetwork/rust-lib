// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyGetLootboxJackpotWinnersInfoParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for DragonsLobbyGetLootboxJackpotWinnersInfoParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for DragonsLobbyGetLootboxJackpotWinnersInfoParams {
    fn topic() -> &'static str {
        "dragons_lobby_getLootboxJackpotWinnersInfo"
    }
    fn method() -> &'static str {
        "lobby_getLootboxJackpotWinnersInfo"
    }
    fn agent() -> &'static str {
        "dragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyGetLootboxJackpotWinnersInfoReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragonsLobbyGetLootboxJackpotWinnersInfoReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetLootboxJackpotWinnersInfoReturns;
impl Schema for DragonsLobbyGetLootboxJackpotWinnersInfoReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragonsLobbyGetLootboxJackpotWinnersInfoReturns {
    fn topic() -> &'static str {
        "dragons_lobby_getLootboxJackpotWinnersInfo"
    }
    fn method() -> &'static str {
        "lobby_getLootboxJackpotWinnersInfo"
    }
    fn agent() -> &'static str {
        "dragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
