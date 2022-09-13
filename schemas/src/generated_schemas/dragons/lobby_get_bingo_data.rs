// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyGetBingoDataParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for DragonsLobbyGetBingoDataParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
impl Agent for DragonsLobbyGetBingoDataParams {
    fn topic() -> &'static str {
        "dragons_lobby_getBingoData"
    }
    fn method() -> &'static str {
        "lobby_getBingoData"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyGetBingoDataReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragonsLobbyGetBingoDataReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetBingoDataReturns;
impl Schema for DragonsLobbyGetBingoDataReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragonsLobbyGetBingoDataReturns {
    fn topic() -> &'static str {
        "dragons_lobby_getBingoData"
    }
    fn method() -> &'static str {
        "lobby_getBingoData"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
