// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for CatsLobbyGetBingoJackpotPoolParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
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
impl Agent for CatsLobbyGetBingoJackpotPoolParams {
    fn topic() -> &'static str {
        "cats_lobby_getBingoJackpotPool"
    }
    fn method() -> &'static str {
        "lobby_getBingoJackpotPool"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetBingoJackpotPoolReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
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
impl Agent for CatsLobbyGetBingoJackpotPoolReturns {
    fn topic() -> &'static str {
        "cats_lobby_getBingoJackpotPool"
    }
    fn method() -> &'static str {
        "lobby_getBingoJackpotPool"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
