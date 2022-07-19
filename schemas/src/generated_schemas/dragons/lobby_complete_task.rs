// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyCompleteTaskParams {
    #[serde(rename = "taskId")]
    pub task_id: i64,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for DragonsLobbyCompleteTaskParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"taskId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"taskId\"]}")
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyCompleteTaskReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragonsLobbyCompleteTaskReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyCompleteTaskReturns;
impl Schema for DragonsLobbyCompleteTaskReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
