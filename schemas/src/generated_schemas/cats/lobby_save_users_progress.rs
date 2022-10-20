// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbySaveUsersProgressParams {
    #[serde(rename = "dataObject", deserialize_with = "deserialize_to_string")]
    pub data_object: String,
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
}
impl Schema for CatsLobbySaveUsersProgressParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"dataObject\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"dataObject\"]}") . unwrap ()
    }
}
impl Agent for CatsLobbySaveUsersProgressParams {
    fn topic() -> &'static str {
        "cats_lobby_saveUsersProgress"
    }
    fn method() -> &'static str {
        "lobby_saveUsersProgress"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for CatsLobbySaveUsersProgressReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbySaveUsersProgressReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbySaveUsersProgressReturns;
impl Schema for CatsLobbySaveUsersProgressReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbySaveUsersProgressReturns {
    fn topic() -> &'static str {
        "cats_lobby_saveUsersProgress"
    }
    fn method() -> &'static str {
        "lobby_saveUsersProgress"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
