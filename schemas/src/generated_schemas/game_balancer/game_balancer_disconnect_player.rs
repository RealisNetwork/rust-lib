// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBalancerGameBalancerDisconnectPlayerParams {
    #[serde(rename = "roomId")]
    pub room_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for GameBalancerGameBalancerDisconnectPlayerParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"roomId\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"roomId\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBalancerGameBalancerDisconnectPlayerReturns(bool);
impl Schema for GameBalancerGameBalancerDisconnectPlayerReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
