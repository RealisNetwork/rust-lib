// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceGetClientStatusByInternalUserIdParams {
    #[serde(rename = "internalUserId", deserialize_with = "deserialize_to_string")]
    pub internal_user_id: String,
}
impl Schema for AuthAuthDeviceGetClientStatusByInternalUserIdParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"internalUserId\":{\"type\":\"string\"}},\"required\":[\"internalUserId\"]}") . unwrap ()
    }
}
impl Agent for AuthAuthDeviceGetClientStatusByInternalUserIdParams {
    fn topic() -> &'static str {
        "auth_authDevice_getClientStatusByInternalUserId"
    }
    fn method() -> &'static str {
        "authDevice_getClientStatusByInternalUserId"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceGetClientStatusByInternalUserIdReturns(pub String);
impl Schema for AuthAuthDeviceGetClientStatusByInternalUserIdReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"string\"}")
    }
}
impl Agent for AuthAuthDeviceGetClientStatusByInternalUserIdReturns {
    fn topic() -> &'static str {
        "auth_authDevice_getClientStatusByInternalUserId"
    }
    fn method() -> &'static str {
        "authDevice_getClientStatusByInternalUserId"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
