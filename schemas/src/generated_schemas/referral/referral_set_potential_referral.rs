// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ReferralReferralSetPotentialReferralParams { # [serde (rename = "referralId" , deserialize_with = "deserialize_to_string")] pub referral_id : String , # [serde (rename = "referrerId" , deserialize_with = "deserialize_to_string")] pub referrer_id : String } impl Schema for ReferralReferralSetPotentialReferralParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"referralId\":{\"type\":\"string\"},\"referrerId\":{\"type\":\"string\"}},\"required\":[\"referralId\",\"referrerId\"]}") . unwrap () } } impl Agent for ReferralReferralSetPotentialReferralParams { fn topic () -> & 'static str { "referral_referral_setPotentialReferral" } fn method () -> & 'static str { "referral_setPotentialReferral" } fn agent () -> & 'static str { "referral" } fn access_level () -> AccessLevel { AccessLevel :: Public } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ReferralReferralSetPotentialReferralReturns (pub bool) ; impl Schema for ReferralReferralSetPotentialReferralReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for ReferralReferralSetPotentialReferralReturns { fn topic () -> & 'static str { "referral_referral_setPotentialReferral" } fn method () -> & 'static str { "referral_setPotentialReferral" } fn agent () -> & 'static str { "referral" } fn access_level () -> AccessLevel { AccessLevel :: Public } }