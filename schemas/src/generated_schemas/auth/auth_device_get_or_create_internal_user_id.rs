// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceGetOrCreateInternalUserIdParams {
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "appId")]
    pub app_id: Option<i32>,
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "provider")]
    pub provider: String,
}
impl Schema for AuthAuthDeviceGetOrCreateInternalUserIdParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"providerId\":{\"type\":\"string\"},\"provider\":{\"type\":\"string\",\"pattern\":\"^(Keycloak)|(DeviceId)$\"}},\"required\":[\"providerId\",\"provider\"]}") . unwrap ()
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
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
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
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
