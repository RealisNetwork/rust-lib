// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyGetProfileInfoParams {
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
}
impl Schema for CatsLobbyGetProfileInfoParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for CatsLobbyGetProfileInfoParams {
    fn topic() -> &'static str {
        "cats_lobby_getProfileInfo"
    }
    fn method() -> &'static str {
        "lobby_getProfileInfo"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetProfileInfoReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyGetProfileInfoReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetProfileInfoReturns;
impl Schema for CatsLobbyGetProfileInfoReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyGetProfileInfoReturns {
    fn topic() -> &'static str {
        "cats_lobby_getProfileInfo"
    }
    fn method() -> &'static str {
        "lobby_getProfileInfo"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
