// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for CatsLobbyGetServerTimeParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CatsLobbyGetServerTimeParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetServerTimeParams;
impl Schema for CatsLobbyGetServerTimeParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetServerTimeReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CatsLobbyGetServerTimeReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetServerTimeReturns;
impl Schema for CatsLobbyGetServerTimeReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
