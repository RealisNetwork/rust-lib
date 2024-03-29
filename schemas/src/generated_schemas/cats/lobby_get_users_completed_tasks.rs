// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyGetUsersCompletedTasksParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsLobbyGetUsersCompletedTasksParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for CatsLobbyGetUsersCompletedTasksParams {
    fn topic() -> &'static str {
        "cats_lobby_getUsersCompletedTasks"
    }
    fn method() -> &'static str {
        "lobby_getUsersCompletedTasks"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetUsersCompletedTasksReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyGetUsersCompletedTasksReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetUsersCompletedTasksReturns;
impl Schema for CatsLobbyGetUsersCompletedTasksReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyGetUsersCompletedTasksReturns {
    fn topic() -> &'static str {
        "cats_lobby_getUsersCompletedTasks"
    }
    fn method() -> &'static str {
        "lobby_getUsersCompletedTasks"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
