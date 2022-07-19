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
impl<'de> Deserialize<'de> for DragonsLobbyGetUsersCompletedTasksReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
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