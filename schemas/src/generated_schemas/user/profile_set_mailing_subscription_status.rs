// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct UserProfileSetMailingSubscriptionStatusParams { # [serde (rename = "status")] pub status : bool } impl Schema for UserProfileSetMailingSubscriptionStatusParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"status\":{\"type\":\"boolean\"}},\"required\":[\"status\"]}") . unwrap () } } impl Agent for UserProfileSetMailingSubscriptionStatusParams { fn topic () -> & 'static str { "user_profile_setMailingSubscriptionStatus" } fn method () -> & 'static str { "profile_setMailingSubscriptionStatus" } fn agent () -> & 'static str { "user" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct UserProfileSetMailingSubscriptionStatusReturns (pub bool) ; impl Schema for UserProfileSetMailingSubscriptionStatusReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for UserProfileSetMailingSubscriptionStatusReturns { fn topic () -> & 'static str { "user_profile_setMailingSubscriptionStatus" } fn method () -> & 'static str { "profile_setMailingSubscriptionStatus" } fn agent () -> & 'static str { "user" } fn access_level () -> AccessLevel { } }