// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct UserProfileBanProfileParams { # [serde (rename = "reason")] pub reason : String , # [serde (rename = "userId")] pub user_id : String } impl Schema for UserProfileBanProfileParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"reason\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"reason\"]}") } } impl Agent for UserProfileBanProfileParams { fn topic () -> & 'static str { "user_profile_banProfile" } fn method () -> & 'static str { "profile_banProfile" } fn agent () -> & 'static str { "user" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct UserProfileBanProfileReturns (bool) ; impl Schema for UserProfileBanProfileReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for UserProfileBanProfileReturns { fn topic () -> & 'static str { "user_profile_banProfile" } fn method () -> & 'static str { "profile_banProfile" } fn agent () -> & 'static str { "user" } }