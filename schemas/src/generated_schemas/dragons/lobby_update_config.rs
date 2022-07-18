// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyUpdateConfigParams {
    #[serde(rename = "configJson")]
    pub config_json: String,
    #[serde(rename = "configKey")]
    pub config_key: String,
}
impl Schema for DragonsLobbyUpdateConfigParams {
    fn schema() -> Value {
        todo!()
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyUpdateConfigReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragonsLobbyUpdateConfigReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyUpdateConfigReturns;
