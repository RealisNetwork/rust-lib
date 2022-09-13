// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyGetBingoJackpotWinnersInfoParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsLobbyGetBingoJackpotWinnersInfoParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
impl Agent for CatsLobbyGetBingoJackpotWinnersInfoParams {
    fn topic() -> &'static str {
        "cats_lobby_getBingoJackpotWinnersInfo"
    }
    fn method() -> &'static str {
        "lobby_getBingoJackpotWinnersInfo"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetBingoJackpotWinnersInfoReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyGetBingoJackpotWinnersInfoReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetBingoJackpotWinnersInfoReturns;
impl Schema for CatsLobbyGetBingoJackpotWinnersInfoReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyGetBingoJackpotWinnersInfoReturns {
    fn topic() -> &'static str {
        "cats_lobby_getBingoJackpotWinnersInfo"
    }
    fn method() -> &'static str {
        "lobby_getBingoJackpotWinnersInfo"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
