// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassSeasonInfoParams {}
impl Schema for BattlePassBattlePassEndpointsGetBattlePassSeasonInfoParams {
    fn schema() -> Value {
        todo!()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassSeasonInfoReturns {
    #[serde(rename = "seasonId")]
    pub season_id: i32,
    #[serde(rename = "stateUpdateDate")]
    pub state_update_date: String,
    #[serde(rename = "seasonState")]
    pub season_state: i32,
    #[serde(rename = "hasNotReceivedOldRewards")]
    pub has_not_received_old_rewards: bool,
}
