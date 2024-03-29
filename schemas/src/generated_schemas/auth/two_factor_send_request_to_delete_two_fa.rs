// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTwoFactorSendRequestToDeleteTwoFaParams(Value);
impl Schema for AuthTwoFactorSendRequestToDeleteTwoFaParams {
    fn schema() -> Value {
        serde_json::from_str("{\"type\":\"object\",\"properties\":{}}").unwrap()
    }
}
impl Agent for AuthTwoFactorSendRequestToDeleteTwoFaParams {
    fn topic() -> &'static str {
        "auth_twoFactor_sendRequestToDeleteTwoFA"
    }
    fn method() -> &'static str {
        "twoFactor_sendRequestToDeleteTwoFA"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTwoFactorSendRequestToDeleteTwoFaReturns(pub bool);
impl Schema for AuthTwoFactorSendRequestToDeleteTwoFaReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthTwoFactorSendRequestToDeleteTwoFaReturns {
    fn topic() -> &'static str {
        "auth_twoFactor_sendRequestToDeleteTwoFA"
    }
    fn method() -> &'static str {
        "twoFactor_sendRequestToDeleteTwoFA"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
