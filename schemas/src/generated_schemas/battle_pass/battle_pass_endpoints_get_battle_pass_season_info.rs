// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassSeasonInfoParams {}
impl Schema for BattlePassBattlePassEndpointsGetBattlePassSeasonInfoParams {
    fn schema() -> Value {
        serde_json::from_str("{\"type\":\"object\",\"properties\":{}}").unwrap()
    }
}
impl Agent for BattlePassBattlePassEndpointsGetBattlePassSeasonInfoParams {
    fn topic() -> &'static str {
        "battle-pass_battlePassEndpoints_getBattlePassSeasonInfo"
    }
    fn method() -> &'static str {
        "battlePassEndpoints_getBattlePassSeasonInfo"
    }
    fn agent() -> &'static str {
        "battle-pass"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassSeasonInfoReturns {
    #[serde(rename = "seasonState")]
    pub season_state: i32,
    #[serde(rename = "hasNotReceivedOldRewards")]
    pub has_not_received_old_rewards: bool,
    #[serde(rename = "stateUpdateDate")]
    pub state_update_date: String,
    #[serde(rename = "seasonId")]
    pub season_id: i32,
}
impl Schema for BattlePassBattlePassEndpointsGetBattlePassSeasonInfoReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"seasonState\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"hasNotReceivedOldRewards\":{\"type\":\"boolean\"},\"stateUpdateDate\":{\"type\":\"string\"},\"seasonId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"seasonId\",\"seasonState\",\"hasNotReceivedOldRewards\",\"stateUpdateDate\"]}")
    }
}
impl Agent for BattlePassBattlePassEndpointsGetBattlePassSeasonInfoReturns {
    fn topic() -> &'static str {
        "battle-pass_battlePassEndpoints_getBattlePassSeasonInfo"
    }
    fn method() -> &'static str {
        "battlePassEndpoints_getBattlePassSeasonInfo"
    }
    fn agent() -> &'static str {
        "battle-pass"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
