// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyRegionsSetParams {
    #[serde(rename = "isFixed")]
    pub is_fixed: bool,
    #[serde(rename = "regionName")]
    pub region_name: String,
}
impl Schema for LobbyRegionsSetParams {
    fn schema() -> Value {
        todo!()
    }
}
impl<'de> Deserialize<'de> for LobbyRegionsSetReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(LobbyRegionsSetReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct LobbyRegionsSetReturns;
