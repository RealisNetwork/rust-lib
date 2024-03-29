// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for AuthAuthSendRequestToDeleteAccountParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(AuthAuthSendRequestToDeleteAccountParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct AuthAuthSendRequestToDeleteAccountParams;
impl Schema for AuthAuthSendRequestToDeleteAccountParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for AuthAuthSendRequestToDeleteAccountParams {
    fn topic() -> &'static str {
        "auth_auth_sendRequestToDeleteAccount"
    }
    fn method() -> &'static str {
        "auth_sendRequestToDeleteAccount"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthSendRequestToDeleteAccountReturns(pub bool);
impl Schema for AuthAuthSendRequestToDeleteAccountReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthAuthSendRequestToDeleteAccountReturns {
    fn topic() -> &'static str {
        "auth_auth_sendRequestToDeleteAccount"
    }
    fn method() -> &'static str {
        "auth_sendRequestToDeleteAccount"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
