// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthTwoFactorVerifyCodeParams { # [serde (rename = "code" , deserialize_with = "deserialize_to_string")] pub code : String } impl Schema for AuthTwoFactorVerifyCodeParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"code\":{\"type\":\"string\"}},\"required\":[\"code\"]}") . unwrap () } } impl Agent for AuthTwoFactorVerifyCodeParams { fn topic () -> & 'static str { "auth_twoFactor_verifyCode" } fn method () -> & 'static str { "twoFactor_verifyCode" } fn agent () -> & 'static str { "auth" } fn access_level () -> AccessLevel { AccessLevel :: Protected } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthTwoFactorVerifyCodeReturns (pub bool) ; impl Schema for AuthTwoFactorVerifyCodeReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for AuthTwoFactorVerifyCodeReturns { fn topic () -> & 'static str { "auth_twoFactor_verifyCode" } fn method () -> & 'static str { "twoFactor_verifyCode" } fn agent () -> & 'static str { "auth" } fn access_level () -> AccessLevel { AccessLevel :: Protected } }