// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBalancerGameBalancerDeletePlayersParams {
    #[serde(rename = "userIds")]
    pub user_ids: Vec<String>,
}
impl Schema for GameBalancerGameBalancerDeletePlayersParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userIds\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}}},\"required\":[\"userIds\"]}") . unwrap ()
    }
}
impl Agent for GameBalancerGameBalancerDeletePlayersParams {
    fn topic() -> &'static str {
        "gameBalancer_gameBalancer_deletePlayers"
    }
    fn method() -> &'static str {
        "gameBalancer_deletePlayers"
    }
    fn agent() -> &'static str {
        "gameBalancer"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBalancerGameBalancerDeletePlayersReturns(pub bool);
impl Schema for GameBalancerGameBalancerDeletePlayersReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for GameBalancerGameBalancerDeletePlayersReturns {
    fn topic() -> &'static str {
        "gameBalancer_gameBalancer_deletePlayers"
    }
    fn method() -> &'static str {
        "gameBalancer_deletePlayers"
    }
    fn agent() -> &'static str {
        "gameBalancer"
    }
}
