// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthConfirmEmailByCodeParams { # [serde (rename = "code")] pub code : String , # [serde (rename = "emailHash")] pub email_hash : String } impl Schema for AuthAuthConfirmEmailByCodeParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"code\":{\"type\":\"string\"},\"emailHash\":{\"type\":\"string\"}},\"required\":[\"code\",\"emailHash\"]}") . unwrap () } } impl Agent for AuthAuthConfirmEmailByCodeParams { fn topic () -> & 'static str { "auth_auth_confirmEmailByCode" } fn method () -> & 'static str { "auth_confirmEmailByCode" } fn agent () -> & 'static str { "auth" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthConfirmEmailByCodeReturns (pub String) ; impl Schema for AuthAuthConfirmEmailByCodeReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"string\"}") } } impl Agent for AuthAuthConfirmEmailByCodeReturns { fn topic () -> & 'static str { "auth_auth_confirmEmailByCode" } fn method () -> & 'static str { "auth_confirmEmailByCode" } fn agent () -> & 'static str { "auth" } fn access_level () -> AccessLevel { } }