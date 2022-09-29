// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthLoginWithFacebookParams {}
impl Schema for AuthAuthLoginWithFacebookParams {
    fn schema() -> Value {
        serde_json::from_str("{\"type\":\"object\",\"properties\":{},\"required\":null}").unwrap()
    }
}
impl Agent for AuthAuthLoginWithFacebookParams {
    fn topic() -> &'static str {
        "auth_auth_loginWithFacebook"
    }
    fn method() -> &'static str {
        "auth_loginWithFacebook"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
impl<'de> Deserialize<'de> for AuthAuthLoginWithFacebookReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(AuthAuthLoginWithFacebookReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct AuthAuthLoginWithFacebookReturns;
impl Schema for AuthAuthLoginWithFacebookReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for AuthAuthLoginWithFacebookReturns {
    fn topic() -> &'static str {
        "auth_auth_loginWithFacebook"
    }
    fn method() -> &'static str {
        "auth_loginWithFacebook"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
