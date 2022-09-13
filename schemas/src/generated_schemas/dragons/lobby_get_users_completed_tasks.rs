// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyGetUsersCompletedTasksParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for DragonsLobbyGetUsersCompletedTasksParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
impl Agent for DragonsLobbyGetUsersCompletedTasksParams {
    fn topic() -> &'static str {
        "dragons_lobby_getUsersCompletedTasks"
    }
    fn method() -> &'static str {
        "lobby_getUsersCompletedTasks"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyGetUsersCompletedTasksReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragonsLobbyGetUsersCompletedTasksReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetUsersCompletedTasksReturns;
impl Schema for DragonsLobbyGetUsersCompletedTasksReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragonsLobbyGetUsersCompletedTasksReturns {
    fn topic() -> &'static str {
        "dragons_lobby_getUsersCompletedTasks"
    }
    fn method() -> &'static str {
        "lobby_getUsersCompletedTasks"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
