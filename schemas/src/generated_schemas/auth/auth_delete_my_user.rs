// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthDeleteMyUserParams { # [serde (rename = "token")] pub token : String } impl Schema for AuthAuthDeleteMyUserParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"token\":{\"type\":\"string\"}},\"required\":null}") } } impl Agent for AuthAuthDeleteMyUserParams { fn topic () -> & 'static str { "auth_auth_deleteMyUser" } fn method () -> & 'static str { "auth_deleteMyUser" } fn agent () -> & 'static str { "auth" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthDeleteMyUserReturns (bool) ; impl Schema for AuthAuthDeleteMyUserReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for AuthAuthDeleteMyUserReturns { fn topic () -> & 'static str { "auth_auth_deleteMyUser" } fn method () -> & 'static str { "auth_deleteMyUser" } fn agent () -> & 'static str { "auth" } }