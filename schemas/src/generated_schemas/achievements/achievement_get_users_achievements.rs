// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementsAchievementGetUsersAchievementsParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for AchievementsAchievementGetUsersAchievementsParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for AchievementsAchievementGetUsersAchievementsParams {
    fn topic() -> &'static str {
        "achievements_achievement_getUsersAchievements"
    }
    fn method() -> &'static str {
        "achievement_getUsersAchievements"
    }
    fn agent() -> &'static str {
        "achievements"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementsAchievementGetUsersAchievementsReturns {
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "Achievements")]
    pub achievements: Vec<String>,
}
impl Schema for AchievementsAchievementGetUsersAchievementsReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"Achievements\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}}},\"required\":[\"status\",\"Achievements\"]}")
    }
}
impl Agent for AchievementsAchievementGetUsersAchievementsReturns {
    fn topic() -> &'static str {
        "achievements_achievement_getUsersAchievements"
    }
    fn method() -> &'static str {
        "achievement_getUsersAchievements"
    }
    fn agent() -> &'static str {
        "achievements"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
