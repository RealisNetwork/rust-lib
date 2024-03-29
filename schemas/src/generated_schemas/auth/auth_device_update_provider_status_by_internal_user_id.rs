// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceUpdateProviderStatusByInternalUserIdParams {
    #[serde(rename = "status")]
    pub status: (),
    #[serde(rename = "provider")]
    pub provider: (),
    #[serde(rename = "internalUserId")]
    pub internal_user_id: String,
}
impl Schema for AuthAuthDeviceUpdateProviderStatusByInternalUserIdParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"status\":{},\"provider\":{},\"internalUserId\":{\"type\":\"string\"}},\"required\":[\"internalUserId\",\"provider\",\"status\"]}")
    }
}
impl Agent for AuthAuthDeviceUpdateProviderStatusByInternalUserIdParams {
    fn topic() -> &'static str {
        "auth_authDevice_updateProviderStatusByInternalUserId"
    }
    fn method() -> &'static str {
        "authDevice_updateProviderStatusByInternalUserId"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceUpdateProviderStatusByInternalUserIdReturns(bool);
impl Schema for AuthAuthDeviceUpdateProviderStatusByInternalUserIdReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthAuthDeviceUpdateProviderStatusByInternalUserIdReturns {
    fn topic() -> &'static str {
        "auth_authDevice_updateProviderStatusByInternalUserId"
    }
    fn method() -> &'static str {
        "authDevice_updateProviderStatusByInternalUserId"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
