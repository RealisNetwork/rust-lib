// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyCompleteTaskParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "taskId")]
    pub task_id: f64,
}
impl Schema for CatsLobbyCompleteTaskParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"taskId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"taskId\"]}") . unwrap ()
    }
}
impl Agent for CatsLobbyCompleteTaskParams {
    fn topic() -> &'static str {
        "cats_lobby_completeTask"
    }
    fn method() -> &'static str {
        "lobby_completeTask"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
impl<'de> Deserialize<'de> for CatsLobbyCompleteTaskReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyCompleteTaskReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyCompleteTaskReturns;
impl Schema for CatsLobbyCompleteTaskReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyCompleteTaskReturns {
    fn topic() -> &'static str {
        "cats_lobby_completeTask"
    }
    fn method() -> &'static str {
        "lobby_completeTask"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
