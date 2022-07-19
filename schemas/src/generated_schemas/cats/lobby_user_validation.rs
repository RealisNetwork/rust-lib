// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyUserValidationParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsLobbyUserValidationParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
impl<'de> Deserialize<'de> for CatsLobbyUserValidationReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CatsLobbyUserValidationReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyUserValidationReturns;
impl Schema for CatsLobbyUserValidationReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}