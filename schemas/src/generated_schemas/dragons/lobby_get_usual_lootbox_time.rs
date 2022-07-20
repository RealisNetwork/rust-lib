// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyGetUsualLootboxTimeParams {}
impl Schema for DragonsLobbyGetUsualLootboxTimeParams {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"object\",\"properties\":{},\"required\":null}")
    }
}
impl Agent for DragonsLobbyGetUsualLootboxTimeParams {
    fn topic() -> &'static str {
        "dragons_lobby_getUsualLootboxTime"
    }
    fn method() -> &'static str {
        "lobby_getUsualLootboxTime"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyGetUsualLootboxTimeReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragonsLobbyGetUsualLootboxTimeReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetUsualLootboxTimeReturns;
impl Schema for DragonsLobbyGetUsualLootboxTimeReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragonsLobbyGetUsualLootboxTimeReturns {
    fn topic() -> &'static str {
        "dragons_lobby_getUsualLootboxTime"
    }
    fn method() -> &'static str {
        "lobby_getUsualLootboxTime"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
