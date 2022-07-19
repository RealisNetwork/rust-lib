// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyGetLootboxJackpotWinnersInfoParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for DragonsLobbyGetLootboxJackpotWinnersInfoParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyGetLootboxJackpotWinnersInfoReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragonsLobbyGetLootboxJackpotWinnersInfoReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetLootboxJackpotWinnersInfoReturns;
impl Schema for DragonsLobbyGetLootboxJackpotWinnersInfoReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}