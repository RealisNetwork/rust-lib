// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyGetTournamentLeaderboardStatsParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsLobbyGetTournamentLeaderboardStatsParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
impl Agent for CatsLobbyGetTournamentLeaderboardStatsParams {
    fn topic() -> &'static str {
        "cats_lobby_getTournamentLeaderboardStats"
    }
    fn method() -> &'static str {
        "lobby_getTournamentLeaderboardStats"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetTournamentLeaderboardStatsReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyGetTournamentLeaderboardStatsReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetTournamentLeaderboardStatsReturns;
impl Schema for CatsLobbyGetTournamentLeaderboardStatsReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyGetTournamentLeaderboardStatsReturns {
    fn topic() -> &'static str {
        "cats_lobby_getTournamentLeaderboardStats"
    }
    fn method() -> &'static str {
        "lobby_getTournamentLeaderboardStats"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
