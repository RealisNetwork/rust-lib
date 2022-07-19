// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceUpdateInternalIdByDeviceIdParams {
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "token")]
    pub token: String,
}
impl Schema for AuthAuthDeviceUpdateInternalIdByDeviceIdParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"deviceId\":{\"type\":\"string\"},\"token\":{\"type\":\"string\"}},\"required\":[\"token\",\"deviceId\"]}")
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
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceUpdateInternalIdByDeviceIdReturns(bool);
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
}
