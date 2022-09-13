// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceGetOrCreateInternalUserIdParams {
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "provider")]
    pub provider: String,
    #[serde(rename = "appId")]
    pub app_id: Option<i32>,
}
impl Schema for AuthAuthDeviceGetOrCreateInternalUserIdParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"providerId\":{\"type\":\"string\"},\"email\":{\"type\":\"string\"},\"provider\":{\"type\":\"string\",\"pattern\":\"^(Keycloak)|(DeviceId)$\"},\"appId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"providerId\",\"provider\"]}")
    }
}
impl Agent for AuthAuthDeviceGetOrCreateInternalUserIdParams {
    fn topic() -> &'static str {
        "auth_authDevice_getOrCreateInternalUserId"
    }
    fn method() -> &'static str {
        "authDevice_getOrCreateInternalUserId"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceGetOrCreateInternalUserIdReturns(pub String);
impl Schema for AuthAuthDeviceGetOrCreateInternalUserIdReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"string\"}")
    }
}
impl Agent for AuthAuthDeviceGetOrCreateInternalUserIdReturns {
    fn topic() -> &'static str {
        "auth_authDevice_getOrCreateInternalUserId"
    }
    fn method() -> &'static str {
        "authDevice_getOrCreateInternalUserId"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
