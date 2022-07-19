// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for CatsLobbyGetBingoJackpotPoolParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CatsLobbyGetBingoJackpotPoolParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetBingoJackpotPoolParams;
impl Schema for CatsLobbyGetBingoJackpotPoolParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetBingoJackpotPoolReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CatsLobbyGetBingoJackpotPoolReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetBingoJackpotPoolReturns;
impl Schema for CatsLobbyGetBingoJackpotPoolReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
