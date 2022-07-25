// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAdminLoginParams {
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "username")]
    pub username: String,
}
impl Schema for AuthAdminLoginParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"password\":{\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\",\"password\"]}")
    }
}
impl Agent for AuthAdminLoginParams {
    fn topic() -> &'static str {
        "auth_admin_login"
    }
    fn method() -> &'static str {
        "admin_login"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAdminLoginReturns {
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
    #[serde(rename = "refresh_expires_in")]
    pub refresh_expires_in: i64,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    #[serde(rename = "access_token")]
    pub access_token: String,
}
impl Schema for AuthAdminLoginReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"expires_in\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"refresh_expires_in\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"refresh_token\":{\"type\":\"string\"},\"access_token\":{\"type\":\"string\"}},\"required\":[\"access_token\",\"expires_in\",\"refresh_expires_in\",\"refresh_token\"]}")
    }
}
impl Agent for AuthAdminLoginReturns {
    fn topic() -> &'static str {
        "auth_admin_login"
    }
    fn method() -> &'static str {
        "admin_login"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
