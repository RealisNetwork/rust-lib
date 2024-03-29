// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for LobbyLobbyStartGameParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(LobbyLobbyStartGameParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct LobbyLobbyStartGameParams;
impl Schema for LobbyLobbyStartGameParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for LobbyLobbyStartGameParams {
    fn topic() -> &'static str {
        "lobby_lobby_startGame"
    }
    fn method() -> &'static str {
        "lobby_startGame"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
impl<'de> Deserialize<'de> for LobbyLobbyStartGameReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(LobbyLobbyStartGameReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct LobbyLobbyStartGameReturns;
impl Schema for LobbyLobbyStartGameReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for LobbyLobbyStartGameReturns {
    fn topic() -> &'static str {
        "lobby_lobby_startGame"
    }
    fn method() -> &'static str {
        "lobby_startGame"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
