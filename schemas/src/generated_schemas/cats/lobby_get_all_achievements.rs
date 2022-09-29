// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for CatsLobbyGetAllAchievementsParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyGetAllAchievementsParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetAllAchievementsParams;
impl Schema for CatsLobbyGetAllAchievementsParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for CatsLobbyGetAllAchievementsParams {
    fn topic() -> &'static str {
        "cats_lobby_getAllAchievements"
    }
    fn method() -> &'static str {
        "lobby_getAllAchievements"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetAllAchievementsReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyGetAllAchievementsReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetAllAchievementsReturns;
impl Schema for CatsLobbyGetAllAchievementsReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyGetAllAchievementsReturns {
    fn topic() -> &'static str {
        "cats_lobby_getAllAchievements"
    }
    fn method() -> &'static str {
        "lobby_getAllAchievements"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
