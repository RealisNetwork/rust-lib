// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct UserProfileGetUserNicknameParams { # [serde (rename = "userId")] pub user_id : String } impl Schema for UserProfileGetUserNicknameParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap () } } impl Agent for UserProfileGetUserNicknameParams { fn topic () -> & 'static str { "user_profile_getUserNickname" } fn method () -> & 'static str { "profile_getUserNickname" } fn agent () -> & 'static str { "user" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct UserProfileGetUserNicknameReturns (pub String) ; impl Schema for UserProfileGetUserNicknameReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"string\"}") } } impl Agent for UserProfileGetUserNicknameReturns { fn topic () -> & 'static str { "user_profile_getUserNickname" } fn method () -> & 'static str { "profile_getUserNickname" } fn agent () -> & 'static str { "user" } fn access_level () -> AccessLevel { } }