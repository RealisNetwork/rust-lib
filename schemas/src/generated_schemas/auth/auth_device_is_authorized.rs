// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceIsAuthorizedParams {
    #[serde(rename = "internalUserId")]
    pub internal_user_id: String,
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "appId")]
    pub app_id: f64,
}
impl Schema for AuthAuthDeviceIsAuthorizedParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"internalUserId\":{\"type\":\"string\"},\"providerId\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"internalUserId\",\"appId\",\"providerId\"]}")
    }
}
impl Agent for AuthAuthDeviceIsAuthorizedParams {
    fn topic() -> &'static str {
        "auth_authDevice_isAuthorized"
    }
    fn method() -> &'static str {
        "authDevice_isAuthorized"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceIsAuthorizedReturns(pub bool);
impl Schema for AuthAuthDeviceIsAuthorizedReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthAuthDeviceIsAuthorizedReturns {
    fn topic() -> &'static str {
        "auth_authDevice_isAuthorized"
    }
    fn method() -> &'static str {
        "authDevice_isAuthorized"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
