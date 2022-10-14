// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthRemoveAccountParams { # [serde (rename = "hash" , deserialize_with = "deserialize_to_string")] pub hash : String } impl Schema for AuthAuthRemoveAccountParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"hash\":{\"type\":\"string\"}},\"required\":[\"hash\"]}") . unwrap () } } impl Agent for AuthAuthRemoveAccountParams { fn topic () -> & 'static str { "auth_auth_removeAccount" } fn method () -> & 'static str { "auth_removeAccount" } fn agent () -> & 'static str { "auth" } fn access_level () -> AccessLevel { AccessLevel :: Public } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthRemoveAccountReturns (pub bool) ; impl Schema for AuthAuthRemoveAccountReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for AuthAuthRemoveAccountReturns { fn topic () -> & 'static str { "auth_auth_removeAccount" } fn method () -> & 'static str { "auth_removeAccount" } fn agent () -> & 'static str { "auth" } fn access_level () -> AccessLevel { AccessLevel :: Public } }