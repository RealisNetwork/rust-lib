// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
impl<'de> Deserialize<'de> for DragonsLobbyGetApplicationSettingsParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragonsLobbyGetApplicationSettingsParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetApplicationSettingsParams;
impl Schema for DragonsLobbyGetApplicationSettingsParams {
    fn schema() -> Value {
        todo!()
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyGetApplicationSettingsReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragonsLobbyGetApplicationSettingsReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetApplicationSettingsReturns;
