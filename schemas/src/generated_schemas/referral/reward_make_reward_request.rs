// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for ReferralRewardMakeRewardRequestParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (ReferralRewardMakeRewardRequestParams) } } # [derive (Debug , Clone , Serialize)] pub struct ReferralRewardMakeRewardRequestParams ; impl Schema for ReferralRewardMakeRewardRequestParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for ReferralRewardMakeRewardRequestParams { fn topic () -> & 'static str { "referral_reward_makeRewardRequest" } fn method () -> & 'static str { "reward_makeRewardRequest" } fn agent () -> & 'static str { "referral" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ReferralRewardMakeRewardRequestReturns (bool) ; impl Schema for ReferralRewardMakeRewardRequestReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for ReferralRewardMakeRewardRequestReturns { fn topic () -> & 'static str { "referral_reward_makeRewardRequest" } fn method () -> & 'static str { "reward_makeRewardRequest" } fn agent () -> & 'static str { "referral" } }