// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct UserProfileUnBanParams { # [serde (rename = "userId")] pub user_id : String } impl Schema for UserProfileUnBanParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap () } } impl Agent for UserProfileUnBanParams { fn topic () -> & 'static str { "user_profile_unBan" } fn method () -> & 'static str { "profile_unBan" } fn agent () -> & 'static str { "user" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct UserProfileUnBanReturns (pub bool) ; impl Schema for UserProfileUnBanReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for UserProfileUnBanReturns { fn topic () -> & 'static str { "user_profile_unBan" } fn method () -> & 'static str { "profile_unBan" } fn agent () -> & 'static str { "user" } fn access_level () -> AccessLevel { } }