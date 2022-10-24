// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassDataParams(Value);
impl Schema for BattlePassBattlePassEndpointsGetBattlePassDataParams {
    fn schema() -> Value {
        serde_json::from_str("{\"type\":\"object\",\"properties\":{}}").unwrap()
    }
}
impl Agent for BattlePassBattlePassEndpointsGetBattlePassDataParams {
    fn topic() -> &'static str {
        "battle-pass_battlePassEndpoints_getBattlePassData"
    }
    fn method() -> &'static str {
        "battlePassEndpoints_getBattlePassData"
    }
    fn agent() -> &'static str {
        "battle-pass"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsBattlePassLevelsParamsParamsBattlePassRewardParamsParamsItemParams
{
    #[serde(rename = "rewardType")]
    pub reward_type: i32,
    #[serde(rename = "amount")]
    pub amount: i32,
    #[serde(rename = "itemId")]
    pub item_id: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsBattlePassLevelsParamsParamsBattlePassRewardParamsParams { # [serde (rename = "battlePassType")] pub battle_pass_type : i32 , # [serde (rename = "item")] pub item : BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsBattlePassLevelsParamsParamsBattlePassRewardParamsParamsItemParams , # [serde (rename = "rewardId")] pub reward_id : i32 }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsBattlePassLevelsParamsParams { # [serde (rename = "experience")] pub experience : i32 , # [serde (rename = "battlePassReward")] pub battle_pass_reward : Vec < BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsBattlePassLevelsParamsParamsBattlePassRewardParamsParams > , # [serde (rename = "level")] pub level : i32 }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsFinalRewardsParamsParamsItemParams
{
    #[serde(rename = "rewardType")]
    pub reward_type: i32,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "amount")]
    pub amount: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsFinalRewardsParamsParams { # [serde (rename = "battlePassType")] pub battle_pass_type : i32 , # [serde (rename = "rewardId")] pub reward_id : i32 , # [serde (rename = "item")] pub item : BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsFinalRewardsParamsParamsItemParams }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParams { # [serde (rename = "battlePassLevels")] pub battle_pass_levels : Vec < BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsBattlePassLevelsParamsParams > , # [serde (rename = "finalRewardsExperienceInterval")] pub final_rewards_experience_interval : i32 , # [serde (rename = "finalRewards")] pub final_rewards : Vec < BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsFinalRewardsParamsParams > }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassDataReturnsRewardsParamsParams {
    #[serde(rename = "rewardId")]
    pub reward_id: i32,
    #[serde(rename = "rewardBindingId")]
    pub reward_binding_id: i32,
    #[serde(rename = "rewardState")]
    pub reward_state: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassDataReturns {
    #[serde(rename = "battlePassProgression")]
    pub battle_pass_progression:
        BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParams,
    #[serde(rename = "rewards")]
    pub rewards: Vec<BattlePassBattlePassEndpointsGetBattlePassDataReturnsRewardsParamsParams>,
}
impl Schema for BattlePassBattlePassEndpointsGetBattlePassDataReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"battlePassProgression\":{\"type\":\"object\",\"properties\":{\"battlePassLevels\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"experience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"battlePassReward\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"battlePassType\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"item\":{\"type\":\"object\",\"properties\":{\"rewardType\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"amount\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"itemId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"rewardType\",\"itemId\",\"amount\"]},\"rewardId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"rewardId\",\"battlePassType\",\"item\"]}},\"level\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"level\",\"experience\",\"battlePassReward\"]}},\"finalRewardsExperienceInterval\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"finalRewards\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"battlePassType\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"rewardId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"item\":{\"type\":\"object\",\"properties\":{\"rewardType\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"itemId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"amount\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"rewardType\",\"itemId\",\"amount\"]}},\"required\":[\"rewardId\",\"battlePassType\",\"item\"]}}},\"required\":[\"battlePassLevels\",\"finalRewardsExperienceInterval\",\"finalRewards\"]},\"rewards\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"rewardId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"rewardBindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"rewardState\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"rewardId\",\"rewardBindingId\",\"rewardState\"]}}},\"required\":[\"battlePassProgression\",\"rewards\"]}")
    }
}
impl Agent for BattlePassBattlePassEndpointsGetBattlePassDataReturns {
    fn topic() -> &'static str {
        "battle-pass_battlePassEndpoints_getBattlePassData"
    }
    fn method() -> &'static str {
        "battlePassEndpoints_getBattlePassData"
    }
    fn agent() -> &'static str {
        "battle-pass"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
