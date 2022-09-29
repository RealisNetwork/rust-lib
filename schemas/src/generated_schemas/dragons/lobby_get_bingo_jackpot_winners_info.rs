// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyGetBingoJackpotWinnersInfoParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for DragonsLobbyGetBingoJackpotWinnersInfoParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for DragonsLobbyGetBingoJackpotWinnersInfoParams {
    fn topic() -> &'static str {
        "dragons_lobby_getBingoJackpotWinnersInfo"
    }
    fn method() -> &'static str {
        "lobby_getBingoJackpotWinnersInfo"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyGetBingoJackpotWinnersInfoReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragonsLobbyGetBingoJackpotWinnersInfoReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetBingoJackpotWinnersInfoReturns;
impl Schema for DragonsLobbyGetBingoJackpotWinnersInfoReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragonsLobbyGetBingoJackpotWinnersInfoReturns {
    fn topic() -> &'static str {
        "dragons_lobby_getBingoJackpotWinnersInfo"
    }
    fn method() -> &'static str {
        "lobby_getBingoJackpotWinnersInfo"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
