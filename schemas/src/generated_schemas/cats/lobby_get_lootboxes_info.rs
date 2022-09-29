// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyGetLootboxesInfoParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsLobbyGetLootboxesInfoParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for CatsLobbyGetLootboxesInfoParams {
    fn topic() -> &'static str {
        "cats_lobby_getLootboxesInfo"
    }
    fn method() -> &'static str {
        "lobby_getLootboxesInfo"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetLootboxesInfoReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyGetLootboxesInfoReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetLootboxesInfoReturns;
impl Schema for CatsLobbyGetLootboxesInfoReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyGetLootboxesInfoReturns {
    fn topic() -> &'static str {
        "cats_lobby_getLootboxesInfo"
    }
    fn method() -> &'static str {
        "lobby_getLootboxesInfo"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
