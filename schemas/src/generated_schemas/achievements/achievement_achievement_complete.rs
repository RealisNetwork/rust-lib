// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementsAchievementAchievementCompleteParams {
    #[serde(rename = "achievementKey")]
    pub achievement_key: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for AchievementsAchievementAchievementCompleteParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"achievementKey\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"achievementKey\",\"userId\"]}") . unwrap ()
    }
}
impl Agent for AchievementsAchievementAchievementCompleteParams {
    fn topic() -> &'static str {
        "achievements_achievement_achievementComplete"
    }
    fn method() -> &'static str {
        "achievement_achievementComplete"
    }
    fn agent() -> &'static str {
        "achievements"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementsAchievementAchievementCompleteReturns {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "status")]
    pub status: i32,
}
impl Schema for AchievementsAchievementAchievementCompleteReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"Key\":{\"type\":\"string\"},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"status\",\"Key\"]}")
    }
}
impl Agent for AchievementsAchievementAchievementCompleteReturns {
    fn topic() -> &'static str {
        "achievements_achievement_achievementComplete"
    }
    fn method() -> &'static str {
        "achievement_achievementComplete"
    }
    fn agent() -> &'static str {
        "achievements"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
