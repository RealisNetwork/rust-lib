// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyUnlinkAccountParams {
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "email")]
    pub email: String,
}
impl Schema for CatsLobbyUnlinkAccountParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"password\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"email\":{\"type\":\"string\"}},\"required\":[\"userId\",\"email\",\"password\"]}")
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
