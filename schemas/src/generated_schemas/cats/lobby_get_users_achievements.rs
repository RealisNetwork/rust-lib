// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyGetUsersAchievementsParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsLobbyGetUsersAchievementsParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for CatsLobbyGetUsersAchievementsParams {
    fn topic() -> &'static str {
        "cats_lobby_getUsersAchievements"
    }
    fn method() -> &'static str {
        "lobby_getUsersAchievements"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetUsersAchievementsReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyGetUsersAchievementsReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetUsersAchievementsReturns;
impl Schema for CatsLobbyGetUsersAchievementsReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyGetUsersAchievementsReturns {
    fn topic() -> &'static str {
        "cats_lobby_getUsersAchievements"
    }
    fn method() -> &'static str {
        "lobby_getUsersAchievements"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
