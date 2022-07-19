// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceAssignInternalIdToProviderIdParams {
    #[serde(rename = "provider")]
    pub provider: (),
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
}
impl Schema for AuthAuthDeviceAssignInternalIdToProviderIdParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"provider\":{},\"providerId\":{\"type\":\"string\"},\"deviceId\":{\"type\":\"string\"}},\"required\":[\"deviceId\",\"providerId\",\"provider\"]}")
    }
}
impl Agent for AuthAuthDeviceAssignInternalIdToProviderIdParams {
    fn topic() -> &'static str {
        "auth_authDevice_assignInternalIdToProviderId"
    }
    fn method() -> &'static str {
        "authDevice_assignInternalIdToProviderId"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceAssignInternalIdToProviderIdReturns(bool);
impl Schema for AuthAuthDeviceAssignInternalIdToProviderIdReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthAuthDeviceAssignInternalIdToProviderIdReturns {
    fn topic() -> &'static str {
        "auth_authDevice_assignInternalIdToProviderId"
    }
    fn method() -> &'static str {
        "authDevice_assignInternalIdToProviderId"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
