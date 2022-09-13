// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyUnlinkAccountParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "email")]
    pub email: String,
}
impl Schema for DragonsLobbyUnlinkAccountParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"password\":{\"type\":\"string\"},\"email\":{\"type\":\"string\"}},\"required\":[\"userId\",\"email\",\"password\"]}")
    }
}
impl Agent for DragonsLobbyUnlinkAccountParams {
    fn topic() -> &'static str {
        "dragons_lobby_unlinkAccount"
    }
    fn method() -> &'static str {
        "lobby_unlinkAccount"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyUnlinkAccountReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragonsLobbyUnlinkAccountReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyUnlinkAccountReturns;
impl Schema for DragonsLobbyUnlinkAccountReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragonsLobbyUnlinkAccountReturns {
    fn topic() -> &'static str {
        "dragons_lobby_unlinkAccount"
    }
    fn method() -> &'static str {
        "lobby_unlinkAccount"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
