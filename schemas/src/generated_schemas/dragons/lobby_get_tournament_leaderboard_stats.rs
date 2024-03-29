// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyGetTournamentLeaderboardStatsParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for DragonsLobbyGetTournamentLeaderboardStatsParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for DragonsLobbyGetTournamentLeaderboardStatsParams {
    fn topic() -> &'static str {
        "dragons_lobby_getTournamentLeaderboardStats"
    }
    fn method() -> &'static str {
        "lobby_getTournamentLeaderboardStats"
    }
    fn agent() -> &'static str {
        "dragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyGetTournamentLeaderboardStatsReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragonsLobbyGetTournamentLeaderboardStatsReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetTournamentLeaderboardStatsReturns;
impl Schema for DragonsLobbyGetTournamentLeaderboardStatsReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragonsLobbyGetTournamentLeaderboardStatsReturns {
    fn topic() -> &'static str {
        "dragons_lobby_getTournamentLeaderboardStats"
    }
    fn method() -> &'static str {
        "lobby_getTournamentLeaderboardStats"
    }
    fn agent() -> &'static str {
        "dragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
