// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsReceiveOldBattlePassRewardsParams {}
impl Schema for BattlePassBattlePassEndpointsReceiveOldBattlePassRewardsParams {
    fn schema() -> Value {
        serde_json::from_str("{\"type\":\"object\",\"properties\":{}}").unwrap()
    }
}
impl Agent for BattlePassBattlePassEndpointsReceiveOldBattlePassRewardsParams {
    fn topic() -> &'static str {
        "battle-pass_battlePassEndpoints_receiveOldBattlePassRewards"
    }
    fn method() -> &'static str {
        "battlePassEndpoints_receiveOldBattlePassRewards"
    }
    fn agent() -> &'static str {
        "battle-pass"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsReceiveOldBattlePassRewardsReturnsBattlePassRewardParamsParamsItemParams
{
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "amount")]
    pub amount: i32,
    #[serde(rename = "rewardType")]
    pub reward_type: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsReceiveOldBattlePassRewardsReturnsBattlePassRewardParamsParams { # [serde (rename = "rewardId")] pub reward_id : i32 , # [serde (rename = "item")] pub item : BattlePassBattlePassEndpointsReceiveOldBattlePassRewardsReturnsBattlePassRewardParamsParamsItemParams , # [serde (rename = "battlePassType")] pub battle_pass_type : i32 }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsReceiveOldBattlePassRewardsReturns {
    #[serde(rename = "battlePassReward")]
    pub battle_pass_reward: Vec<
        BattlePassBattlePassEndpointsReceiveOldBattlePassRewardsReturnsBattlePassRewardParamsParams,
    >,
}
impl Schema for BattlePassBattlePassEndpointsReceiveOldBattlePassRewardsReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"battlePassReward\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"rewardId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"item\":{\"type\":\"object\",\"properties\":{\"itemId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"amount\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"rewardType\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"rewardType\",\"itemId\",\"amount\"]},\"battlePassType\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"rewardId\",\"battlePassType\",\"item\"]}}},\"required\":[\"battlePassReward\"]}")
    }
}
impl Agent for BattlePassBattlePassEndpointsReceiveOldBattlePassRewardsReturns {
    fn topic() -> &'static str {
        "battle-pass_battlePassEndpoints_receiveOldBattlePassRewards"
    }
    fn method() -> &'static str {
        "battlePassEndpoints_receiveOldBattlePassRewards"
    }
    fn agent() -> &'static str {
        "battle-pass"
    }
}
