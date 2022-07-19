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
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
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
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementsAchievementGetUsersAchievementsReturnsParamsGameEventParams {
    #[serde(rename = "eventKey")]
    pub event_key: String,
    #[serde(rename = "eventLifeType")]
    pub event_life_type: i32,
    #[serde(rename = "count")]
    pub count: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementsAchievementGetUsersAchievementsReturnsParams {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "reward")]
    pub reward: i64,
    #[serde(rename = "achievementKey")]
    pub achievement_key: String,
    #[serde(rename = "rewardType")]
    pub reward_type: i32,
    #[serde(rename = "gameEvent")]
    pub game_event: AchievementsAchievementGetUsersAchievementsReturnsParamsGameEventParams,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementsAchievementGetUsersAchievementsReturns(
    Vec<AchievementsAchievementGetUsersAchievementsReturnsParams>,
);
impl Schema for AchievementsAchievementGetUsersAchievementsReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"description\":{\"type\":\"string\"},\"reward\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"achievementKey\":{\"type\":\"string\"},\"rewardType\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"gameEvent\":{\"type\":\"object\",\"properties\":{\"eventKey\":{\"type\":\"string\"},\"eventLifeType\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"count\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"eventKey\",\"eventLifeType\",\"count\"]}},\"required\":[\"achievementKey\",\"rewardType\",\"reward\",\"description\",\"gameEvent\"]}}")
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
}
