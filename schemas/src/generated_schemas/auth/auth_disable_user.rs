// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthDisableUserParams { # [serde (rename = "userId" , deserialize_with = "deserialize_to_string")] pub user_id : String } impl Schema for AuthAuthDisableUserParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap () } } impl Agent for AuthAuthDisableUserParams { fn topic () -> & 'static str { "auth_auth_disableUser" } fn method () -> & 'static str { "auth_disableUser" } fn agent () -> & 'static str { "auth" } fn access_level () -> AccessLevel { AccessLevel :: Internal } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthDisableUserReturns (pub bool) ; impl Schema for AuthAuthDisableUserReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for AuthAuthDisableUserReturns { fn topic () -> & 'static str { "auth_auth_disableUser" } fn method () -> & 'static str { "auth_disableUser" } fn agent () -> & 'static str { "auth" } fn access_level () -> AccessLevel { AccessLevel :: Internal } }