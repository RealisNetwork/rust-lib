// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyUnlinkAccountParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "password")]
    pub password: String,
}
impl Schema for CatsLobbyUnlinkAccountParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"email\":{\"type\":\"string\"},\"password\":{\"type\":\"string\"}},\"required\":[\"userId\",\"email\",\"password\"]}")
    }
}
impl Agent for CatsLobbyUnlinkAccountParams {
    fn topic() -> &'static str {
        "cats_lobby_unlinkAccount"
    }
    fn method() -> &'static str {
        "lobby_unlinkAccount"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
impl<'de> Deserialize<'de> for CatsLobbyUnlinkAccountReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CatsLobbyUnlinkAccountReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyUnlinkAccountReturns;
impl Schema for CatsLobbyUnlinkAccountReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyUnlinkAccountReturns {
    fn topic() -> &'static str {
        "cats_lobby_unlinkAccount"
    }
    fn method() -> &'static str {
        "lobby_unlinkAccount"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
