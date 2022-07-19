// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for CatsLobbyGetClientVersionParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CatsLobbyGetClientVersionParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetClientVersionParams;
impl Schema for CatsLobbyGetClientVersionParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetClientVersionReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CatsLobbyGetClientVersionReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetClientVersionReturns;
impl Schema for CatsLobbyGetClientVersionReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}