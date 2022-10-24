// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBalancerGameBalancerDisconnectPlayerParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "roomId")]
    pub room_id: String,
}
impl Schema for GameBalancerGameBalancerDisconnectPlayerParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"roomId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"roomId\"]}") . unwrap ()
    }
}
impl Agent for GameBalancerGameBalancerDisconnectPlayerParams {
    fn topic() -> &'static str {
        "gameBalancer_gameBalancer_disconnectPlayer"
    }
    fn method() -> &'static str {
        "gameBalancer_disconnectPlayer"
    }
    fn agent() -> &'static str {
        "gameBalancer"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBalancerGameBalancerDisconnectPlayerReturns(pub bool);
impl Schema for GameBalancerGameBalancerDisconnectPlayerReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for GameBalancerGameBalancerDisconnectPlayerReturns {
    fn topic() -> &'static str {
        "gameBalancer_gameBalancer_disconnectPlayer"
    }
    fn method() -> &'static str {
        "gameBalancer_disconnectPlayer"
    }
    fn agent() -> &'static str {
        "gameBalancer"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
