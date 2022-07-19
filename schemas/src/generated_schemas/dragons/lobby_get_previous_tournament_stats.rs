// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for DragonsLobbyGetPreviousTournamentStatsParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragonsLobbyGetPreviousTournamentStatsParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetPreviousTournamentStatsParams;
impl Schema for DragonsLobbyGetPreviousTournamentStatsParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyGetPreviousTournamentStatsReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragonsLobbyGetPreviousTournamentStatsReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetPreviousTournamentStatsReturns;
impl Schema for DragonsLobbyGetPreviousTournamentStatsReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
