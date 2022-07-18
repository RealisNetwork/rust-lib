// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
impl<'de> Deserialize<'de> for DragonsLobbyGetPreviousTournamentStatsParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
        todo!()
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyGetPreviousTournamentStatsReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragonsLobbyGetPreviousTournamentStatsReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetPreviousTournamentStatsReturns;
