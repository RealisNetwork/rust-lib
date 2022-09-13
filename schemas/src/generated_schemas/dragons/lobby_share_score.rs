// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyShareScoreParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "score")]
    pub score: String,
}
impl Schema for DragonsLobbyShareScoreParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"score\":{\"type\":\"string\"}},\"required\":[\"userId\",\"score\"]}")
    }
}
impl Agent for DragonsLobbyShareScoreParams {
    fn topic() -> &'static str {
        "dragons_lobby_shareScore"
    }
    fn method() -> &'static str {
        "lobby_shareScore"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyShareScoreReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragonsLobbyShareScoreReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyShareScoreReturns;
impl Schema for DragonsLobbyShareScoreReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragonsLobbyShareScoreReturns {
    fn topic() -> &'static str {
        "dragons_lobby_shareScore"
    }
    fn method() -> &'static str {
        "lobby_shareScore"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
