// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for CatsLobbyGetLootboxJackpotPoolParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyGetLootboxJackpotPoolParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetLootboxJackpotPoolParams;
impl Schema for CatsLobbyGetLootboxJackpotPoolParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for CatsLobbyGetLootboxJackpotPoolParams {
    fn topic() -> &'static str {
        "cats_lobby_getLootboxJackpotPool"
    }
    fn method() -> &'static str {
        "lobby_getLootboxJackpotPool"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetLootboxJackpotPoolReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyGetLootboxJackpotPoolReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetLootboxJackpotPoolReturns;
impl Schema for CatsLobbyGetLootboxJackpotPoolReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyGetLootboxJackpotPoolReturns {
    fn topic() -> &'static str {
        "cats_lobby_getLootboxJackpotPool"
    }
    fn method() -> &'static str {
        "lobby_getLootboxJackpotPool"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
