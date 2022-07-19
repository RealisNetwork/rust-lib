// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyUnlinkAccountParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "password")]
    pub password: String,
}
impl Schema for DragonsLobbyUnlinkAccountParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"email\":{\"type\":\"string\"},\"password\":{\"type\":\"string\"}},\"required\":[\"userId\",\"email\",\"password\"]}")
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyUnlinkAccountReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
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