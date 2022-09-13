// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTwoFactorVerifyCodeParams {
    #[serde(rename = "code")]
    pub code: String,
}
impl Schema for AuthTwoFactorVerifyCodeParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"code\":{\"type\":\"string\"}},\"required\":[\"code\"]}")
    }
}
impl Agent for AuthTwoFactorVerifyCodeParams {
    fn topic() -> &'static str {
        "auth_twoFactor_verifyCode"
    }
    fn method() -> &'static str {
        "twoFactor_verifyCode"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTwoFactorVerifyCodeReturns(pub bool);
impl Schema for AuthTwoFactorVerifyCodeReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthTwoFactorVerifyCodeReturns {
    fn topic() -> &'static str {
        "auth_twoFactor_verifyCode"
    }
    fn method() -> &'static str {
        "twoFactor_verifyCode"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
