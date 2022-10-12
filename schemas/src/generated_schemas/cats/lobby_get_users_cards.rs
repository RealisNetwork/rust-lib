// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyGetUsersCardsParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsLobbyGetUsersCardsParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for CatsLobbyGetUsersCardsParams {
    fn topic() -> &'static str {
        "cats_lobby_getUsersCards"
    }
    fn method() -> &'static str {
        "lobby_getUsersCards"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetUsersCardsReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyGetUsersCardsReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetUsersCardsReturns;
impl Schema for CatsLobbyGetUsersCardsReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyGetUsersCardsReturns {
    fn topic() -> &'static str {
        "cats_lobby_getUsersCards"
    }
    fn method() -> &'static str {
        "lobby_getUsersCards"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
