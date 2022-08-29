// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTwoFactorGetStatusParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for AuthTwoFactorGetStatusParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
impl Agent for AuthTwoFactorGetStatusParams {
    fn topic() -> &'static str {
        "auth_twoFactor_getStatus"
    }
    fn method() -> &'static str {
        "twoFactor_getStatus"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTwoFactorGetStatusReturns(pub bool);
impl Schema for AuthTwoFactorGetStatusReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthTwoFactorGetStatusReturns {
    fn topic() -> &'static str {
        "auth_twoFactor_getStatus"
    }
    fn method() -> &'static str {
        "twoFactor_getStatus"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
