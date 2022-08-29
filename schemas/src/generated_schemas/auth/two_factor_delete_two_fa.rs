// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTwoFactorDeleteTwoFaParams {
    #[serde(rename = "hash")]
    pub hash: String,
}
impl Schema for AuthTwoFactorDeleteTwoFaParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"hash\":{\"type\":\"string\"}},\"required\":[\"hash\"]}")
    }
}
impl Agent for AuthTwoFactorDeleteTwoFaParams {
    fn topic() -> &'static str {
        "auth_twoFactor_deleteTwoFA"
    }
    fn method() -> &'static str {
        "twoFactor_deleteTwoFA"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTwoFactorDeleteTwoFaReturns(pub bool);
impl Schema for AuthTwoFactorDeleteTwoFaReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthTwoFactorDeleteTwoFaReturns {
    fn topic() -> &'static str {
        "auth_twoFactor_deleteTwoFA"
    }
    fn method() -> &'static str {
        "twoFactor_deleteTwoFA"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
