// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BattlePassBattlePassEndpointsPurchaseBattlePassExperienceToLevelupParams { } impl Schema for BattlePassBattlePassEndpointsPurchaseBattlePassExperienceToLevelupParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{}}") . unwrap () } } impl Agent for BattlePassBattlePassEndpointsPurchaseBattlePassExperienceToLevelupParams { fn topic () -> & 'static str { "battle-pass_battlePassEndpoints_purchaseBattlePassExperienceToLevelup" } fn method () -> & 'static str { "battlePassEndpoints_purchaseBattlePassExperienceToLevelup" } fn agent () -> & 'static str { "battle-pass" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BattlePassBattlePassEndpointsPurchaseBattlePassExperienceToLevelupReturns { } impl Schema for BattlePassBattlePassEndpointsPurchaseBattlePassExperienceToLevelupReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{}}") } } impl Agent for BattlePassBattlePassEndpointsPurchaseBattlePassExperienceToLevelupReturns { fn topic () -> & 'static str { "battle-pass_battlePassEndpoints_purchaseBattlePassExperienceToLevelup" } fn method () -> & 'static str { "battlePassEndpoints_purchaseBattlePassExperienceToLevelup" } fn agent () -> & 'static str { "battle-pass" } fn access_level () -> AccessLevel { } }