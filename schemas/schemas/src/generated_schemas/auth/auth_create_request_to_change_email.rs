// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthCreateRequestToChangeEmailParams { # [serde (rename = "password")] pub password : String , # [serde (rename = "newEmail")] pub new_email : String } impl Schema for AuthAuthCreateRequestToChangeEmailParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"password\":{\"type\":\"string\"},\"newEmail\":{\"type\":\"string\"}},\"required\":[\"newEmail\",\"password\"]}") } } impl Agent for AuthAuthCreateRequestToChangeEmailParams { fn topic () -> & 'static str { "auth_auth_createRequestToChangeEmail" } fn method () -> & 'static str { "auth_createRequestToChangeEmail" } fn agent () -> & 'static str { "auth" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthCreateRequestToChangeEmailReturns (bool) ; impl Schema for AuthAuthCreateRequestToChangeEmailReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for AuthAuthCreateRequestToChangeEmailReturns { fn topic () -> & 'static str { "auth_auth_createRequestToChangeEmail" } fn method () -> & 'static str { "auth_createRequestToChangeEmail" } fn agent () -> & 'static str { "auth" } }