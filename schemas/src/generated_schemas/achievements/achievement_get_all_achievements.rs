// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for AchievementsAchievementGetAllAchievementsParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(AchievementsAchievementGetAllAchievementsParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct AchievementsAchievementGetAllAchievementsParams;
impl Schema for AchievementsAchievementGetAllAchievementsParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for AchievementsAchievementGetAllAchievementsParams {
    fn topic() -> &'static str {
        "achievements_achievement_getAllAchievements"
    }
    fn method() -> &'static str {
        "achievement_getAllAchievements"
    }
    fn agent() -> &'static str {
        "achievements"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementsAchievementGetAllAchievementsReturnsAchievementsParamsParamsGameEventsParamsParams
{
    #[serde(rename = "EventLifeType")]
    pub event_life_type: String,
    #[serde(rename = "Count")]
    pub count: f64,
    #[serde(rename = "EventKey")]
    pub event_key: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementsAchievementGetAllAchievementsReturnsAchievementsParamsParamsDescriptionParamsParams
{
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "Language")]
    pub language: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementsAchievementGetAllAchievementsReturnsAchievementsParamsParams { # [serde (rename = "Reward")] pub reward : f64 , # [serde (rename = "GameEvents")] pub game_events : Vec < AchievementsAchievementGetAllAchievementsReturnsAchievementsParamsParamsGameEventsParamsParams > , # [serde (rename = "Description")] pub description : Vec < AchievementsAchievementGetAllAchievementsReturnsAchievementsParamsParamsDescriptionParamsParams > , # [serde (rename = "RewardType")] pub reward_type : i32 , # [serde (rename = "Key")] pub key : String }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementsAchievementGetAllAchievementsReturns {
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "Achievements")]
    pub achievements: Vec<AchievementsAchievementGetAllAchievementsReturnsAchievementsParamsParams>,
}
impl Schema for AchievementsAchievementGetAllAchievementsReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"Achievements\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"Reward\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"GameEvents\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"EventLifeType\":{\"type\":\"string\"},\"Count\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"EventKey\":{\"type\":\"string\"}},\"required\":[\"EventKey\",\"EventLifeType\",\"Count\"]}},\"Description\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"Text\":{\"type\":\"string\"},\"Language\":{\"type\":\"string\"}},\"required\":[\"Language\",\"Text\"]}},\"RewardType\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"Key\":{\"type\":\"string\"}},\"required\":[\"Key\",\"RewardType\",\"Reward\",\"Description\",\"GameEvents\"]}}},\"required\":[\"status\",\"Achievements\"]}")
    }
}
impl Agent for AchievementsAchievementGetAllAchievementsReturns {
    fn topic() -> &'static str {
        "achievements_achievement_getAllAchievements"
    }
    fn method() -> &'static str {
        "achievement_getAllAchievements"
    }
    fn agent() -> &'static str {
        "achievements"
    }
}
