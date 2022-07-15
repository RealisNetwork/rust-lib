// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementsAchievementGetUsersAchievementsParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementsAchievementGetUsersAchievementsReturnsParamsGameEventParams {
    #[serde(rename = "eventKey")]
    pub event_key: String,
    #[serde(rename = "eventLifeType")]
    pub event_life_type: i32,
    #[serde(rename = "count")]
    pub count: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementsAchievementGetUsersAchievementsReturnsParams {
    #[serde(rename = "achievementKey")]
    pub achievement_key: String,
    #[serde(rename = "reward")]
    pub reward: i64,
    #[serde(rename = "gameEvent")]
    pub game_event: AchievementsAchievementGetUsersAchievementsReturnsParamsGameEventParams,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "rewardType")]
    pub reward_type: i32,
}
pub type AchievementsAchievementGetUsersAchievementsReturns =
    Vec<AchievementsAchievementGetUsersAchievementsReturnsParams>;
