// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTwoFactorDeleteSecretParams {
    #[serde(rename = "token")]
    pub token: String,
}
impl Schema for AuthTwoFactorDeleteSecretParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"token\":{\"type\":\"string\"}},\"required\":[\"token\"]}")
    }
}
impl Agent for AuthTwoFactorDeleteSecretParams {
    fn topic() -> &'static str {
        "auth_twoFactor_deleteSecret"
    }
    fn method() -> &'static str {
        "twoFactor_deleteSecret"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTwoFactorDeleteSecretReturns(bool);
impl Schema for AuthTwoFactorDeleteSecretReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthTwoFactorDeleteSecretReturns {
    fn topic() -> &'static str {
        "auth_twoFactor_deleteSecret"
    }
    fn method() -> &'static str {
        "twoFactor_deleteSecret"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
