// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyGetUsersAchievementsParams {
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
}
impl Schema for DragonsLobbyGetUsersAchievementsParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for DragonsLobbyGetUsersAchievementsParams {
    fn topic() -> &'static str {
        "dragons_lobby_getUsersAchievements"
    }
    fn method() -> &'static str {
        "lobby_getUsersAchievements"
    }
    fn agent() -> &'static str {
        "dragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyGetUsersAchievementsReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragonsLobbyGetUsersAchievementsReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetUsersAchievementsReturns;
impl Schema for DragonsLobbyGetUsersAchievementsReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragonsLobbyGetUsersAchievementsReturns {
    fn topic() -> &'static str {
        "dragons_lobby_getUsersAchievements"
    }
    fn method() -> &'static str {
        "lobby_getUsersAchievements"
    }
    fn agent() -> &'static str {
        "dragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
