// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for AchievementsAchievementGetAllAchievementsParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (AchievementsAchievementGetAllAchievementsParams) } } # [derive (Debug , Clone , Serialize)] pub struct AchievementsAchievementGetAllAchievementsParams ; impl Schema for AchievementsAchievementGetAllAchievementsParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for AchievementsAchievementGetAllAchievementsParams { fn topic () -> & 'static str { "achievements_achievement_getAllAchievements" } fn method () -> & 'static str { "achievement_getAllAchievements" } fn agent () -> & 'static str { "achievements" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AchievementsAchievementGetAllAchievementsReturnsParamsGameEventParams { # [serde (rename = "eventLifeType")] pub event_life_type : i32 , # [serde (rename = "eventKey")] pub event_key : String , # [serde (rename = "count")] pub count : i64 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AchievementsAchievementGetAllAchievementsReturnsParams { # [serde (rename = "reward")] pub reward : i64 , # [serde (rename = "rewardType")] pub reward_type : i32 , # [serde (rename = "gameEvent")] pub game_event : AchievementsAchievementGetAllAchievementsReturnsParamsGameEventParams , # [serde (rename = "description")] pub description : String , # [serde (rename = "achievementKey")] pub achievement_key : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AchievementsAchievementGetAllAchievementsReturns (Vec < AchievementsAchievementGetAllAchievementsReturnsParams >) ; impl Schema for AchievementsAchievementGetAllAchievementsReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"reward\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"rewardType\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"gameEvent\":{\"type\":\"object\",\"properties\":{\"eventLifeType\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"eventKey\":{\"type\":\"string\"},\"count\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"eventKey\",\"eventLifeType\",\"count\"]},\"description\":{\"type\":\"string\"},\"achievementKey\":{\"type\":\"string\"}},\"required\":[\"achievementKey\",\"rewardType\",\"reward\",\"description\",\"gameEvent\"]}}") } } impl Agent for AchievementsAchievementGetAllAchievementsReturns { fn topic () -> & 'static str { "achievements_achievement_getAllAchievements" } fn method () -> & 'static str { "achievement_getAllAchievements" } fn agent () -> & 'static str { "achievements" } }