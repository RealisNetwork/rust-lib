// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyGetOffersAndBonusesParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsLobbyGetOffersAndBonusesParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
impl Agent for CatsLobbyGetOffersAndBonusesParams {
    fn topic() -> &'static str {
        "cats_lobby_getOffersAndBonuses"
    }
    fn method() -> &'static str {
        "lobby_getOffersAndBonuses"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetOffersAndBonusesReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyGetOffersAndBonusesReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetOffersAndBonusesReturns;
impl Schema for CatsLobbyGetOffersAndBonusesReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyGetOffersAndBonusesReturns {
    fn topic() -> &'static str {
        "cats_lobby_getOffersAndBonuses"
    }
    fn method() -> &'static str {
        "lobby_getOffersAndBonuses"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
