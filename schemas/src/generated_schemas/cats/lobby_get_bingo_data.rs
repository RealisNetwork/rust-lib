// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyGetBingoDataParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsLobbyGetBingoDataParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetBingoDataReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CatsLobbyGetBingoDataReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetBingoDataReturns;
impl Schema for CatsLobbyGetBingoDataReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
