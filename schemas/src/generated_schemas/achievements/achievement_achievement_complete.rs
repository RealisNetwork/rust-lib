// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementsAchievementAchievementCompleteParams {
    #[serde(rename = "achievementKey")]
    pub achievement_key: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementsAchievementAchievementCompleteReturns {
    #[serde(rename = "achievementKey")]
    pub achievement_key: String,
}
