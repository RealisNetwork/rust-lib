// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceUpdateInternalIdByDeviceIdParams {
    #[serde(rename = "token", deserialize_with = "deserialize_to_string")]
    pub token: String,
    #[serde(rename = "deviceId", deserialize_with = "deserialize_to_string")]
    pub device_id: String,
}
impl Schema for AuthAuthDeviceUpdateInternalIdByDeviceIdParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"token\":{\"type\":\"string\"},\"deviceId\":{\"type\":\"string\"}},\"required\":[\"token\",\"deviceId\"]}") . unwrap ()
    }
}
impl Agent for AuthAuthDeviceUpdateInternalIdByDeviceIdParams {
    fn topic() -> &'static str {
        "auth_authDevice_updateInternalIdByDeviceId"
    }
    fn method() -> &'static str {
        "authDevice_updateInternalIdByDeviceId"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceUpdateInternalIdByDeviceIdReturns(pub bool);
impl Schema for AuthAuthDeviceUpdateInternalIdByDeviceIdReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthAuthDeviceUpdateInternalIdByDeviceIdReturns {
    fn topic() -> &'static str {
        "auth_authDevice_updateInternalIdByDeviceId"
    }
    fn method() -> &'static str {
        "authDevice_updateInternalIdByDeviceId"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
