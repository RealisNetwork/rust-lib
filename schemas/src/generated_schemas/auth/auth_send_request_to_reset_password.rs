// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthSendRequestToResetPasswordParams {
    #[serde(rename = "email")]
    pub email: String,
}
impl Schema for AuthAuthSendRequestToResetPasswordParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\",\"pattern\":\"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\\\.[a-zA-Z0-9-.]+$\"}},\"required\":[\"email\"]}") . unwrap ()
    }
}
impl Agent for AuthAuthSendRequestToResetPasswordParams {
    fn topic() -> &'static str {
        "auth_auth_sendRequestToResetPassword"
    }
    fn method() -> &'static str {
        "auth_sendRequestToResetPassword"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthSendRequestToResetPasswordReturns(pub bool);
impl Schema for AuthAuthSendRequestToResetPasswordReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthAuthSendRequestToResetPasswordReturns {
    fn topic() -> &'static str {
        "auth_auth_sendRequestToResetPassword"
    }
    fn method() -> &'static str {
        "auth_sendRequestToResetPassword"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
