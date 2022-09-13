// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyGetAdventureMapStatsParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for DragonsLobbyGetAdventureMapStatsParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
impl Agent for DragonsLobbyGetAdventureMapStatsParams {
    fn topic() -> &'static str {
        "dragons_lobby_getAdventureMapStats"
    }
    fn method() -> &'static str {
        "lobby_getAdventureMapStats"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyGetAdventureMapStatsReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragonsLobbyGetAdventureMapStatsReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetAdventureMapStatsReturns;
impl Schema for DragonsLobbyGetAdventureMapStatsReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragonsLobbyGetAdventureMapStatsReturns {
    fn topic() -> &'static str {
        "dragons_lobby_getAdventureMapStats"
    }
    fn method() -> &'static str {
        "lobby_getAdventureMapStats"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
