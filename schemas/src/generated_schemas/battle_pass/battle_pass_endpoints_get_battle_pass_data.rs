// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassDataParams {}
#[derive(Debug, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsBattlePassLevelsParamsParamsBattlePassRewardParamsParamsItemParams
{
    #[serde(rename = "rewardType")]
    pub reward_type: i32,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "amount")]
    pub amount: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsBattlePassLevelsParamsParamsBattlePassRewardParamsParams { # [serde (rename = "item")] pub item : BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsBattlePassLevelsParamsParamsBattlePassRewardParamsParamsItemParams , # [serde (rename = "rewardId")] pub reward_id : i32 , # [serde (rename = "battlePassType")] pub battle_pass_type : i32 }
#[derive(Debug, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsBattlePassLevelsParamsParams { # [serde (rename = "experience")] pub experience : i32 , # [serde (rename = "battlePassReward")] pub battle_pass_reward : Vec < BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsBattlePassLevelsParamsParamsBattlePassRewardParamsParams > , # [serde (rename = "level")] pub level : i32 }
#[derive(Debug, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsFinalRewardsParamsParamsItemParams
{
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "amount")]
    pub amount: i32,
    #[serde(rename = "rewardType")]
    pub reward_type: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsFinalRewardsParamsParams { # [serde (rename = "battlePassType")] pub battle_pass_type : i32 , # [serde (rename = "rewardId")] pub reward_id : i32 , # [serde (rename = "item")] pub item : BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsFinalRewardsParamsParamsItemParams }
#[derive(Debug, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParams { # [serde (rename = "battlePassLevels")] pub battle_pass_levels : Vec < BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsBattlePassLevelsParamsParams > , # [serde (rename = "finalRewards")] pub final_rewards : Vec < BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParamsFinalRewardsParamsParams > , # [serde (rename = "finalRewardsExperienceInterval")] pub final_rewards_experience_interval : i32 }
#[derive(Debug, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassDataReturnsRewardsParamsParams {
    #[serde(rename = "rewardState")]
    pub reward_state: i32,
    #[serde(rename = "rewardId")]
    pub reward_id: i32,
    #[serde(rename = "rewardBindingId")]
    pub reward_binding_id: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassDataReturns {
    #[serde(rename = "battlePassProgression")]
    pub battle_pass_progression:
        BattlePassBattlePassEndpointsGetBattlePassDataReturnsBattlePassProgressionParams,
    #[serde(rename = "rewards")]
    pub rewards: Vec<BattlePassBattlePassEndpointsGetBattlePassDataReturnsRewardsParamsParams>,
}
