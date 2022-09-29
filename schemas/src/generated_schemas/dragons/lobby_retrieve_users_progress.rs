// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyRetrieveUsersProgressParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for DragonsLobbyRetrieveUsersProgressParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for DragonsLobbyRetrieveUsersProgressParams {
    fn topic() -> &'static str {
        "dragons_lobby_retrieveUsersProgress"
    }
    fn method() -> &'static str {
        "lobby_retrieveUsersProgress"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyRetrieveUsersProgressReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragonsLobbyRetrieveUsersProgressReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyRetrieveUsersProgressReturns;
impl Schema for DragonsLobbyRetrieveUsersProgressReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragonsLobbyRetrieveUsersProgressReturns {
    fn topic() -> &'static str {
        "dragons_lobby_retrieveUsersProgress"
    }
    fn method() -> &'static str {
        "lobby_retrieveUsersProgress"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
