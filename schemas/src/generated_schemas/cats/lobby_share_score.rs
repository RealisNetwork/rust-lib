// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyShareScoreParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "score")]
    pub score: String,
}
impl Schema for CatsLobbyShareScoreParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"score\":{\"type\":\"string\"}},\"required\":[\"userId\",\"score\"]}")
    }
}
impl<'de> Deserialize<'de> for CatsLobbyShareScoreReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CatsLobbyShareScoreReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyShareScoreReturns;
impl Schema for CatsLobbyShareScoreReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
