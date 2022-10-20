// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyAdventureMapLevelUpParams {
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
}
impl Schema for CatsLobbyAdventureMapLevelUpParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for CatsLobbyAdventureMapLevelUpParams {
    fn topic() -> &'static str {
        "cats_lobby_adventureMapLevelUp"
    }
    fn method() -> &'static str {
        "lobby_adventureMapLevelUp"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for CatsLobbyAdventureMapLevelUpReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyAdventureMapLevelUpReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyAdventureMapLevelUpReturns;
impl Schema for CatsLobbyAdventureMapLevelUpReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyAdventureMapLevelUpReturns {
    fn topic() -> &'static str {
        "cats_lobby_adventureMapLevelUp"
    }
    fn method() -> &'static str {
        "lobby_adventureMapLevelUp"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
