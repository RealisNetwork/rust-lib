// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthChangePasswordParams {
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "newPassword")]
    pub new_password: String,
    #[serde(rename = "username")]
    pub username: String,
}
impl Schema for AuthAuthChangePasswordParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"password\":{\"type\":\"string\"},\"newPassword\":{\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"password\",\"username\",\"newPassword\"]}")
    }
}
impl Agent for AuthAuthChangePasswordParams {
    fn topic() -> &'static str {
        "auth_auth_changePassword"
    }
    fn method() -> &'static str {
        "auth_changePassword"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthChangePasswordReturns(bool);
impl Schema for AuthAuthChangePasswordReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthAuthChangePasswordReturns {
    fn topic() -> &'static str {
        "auth_auth_changePassword"
    }
    fn method() -> &'static str {
        "auth_changePassword"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
