// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceUpdateProviderStatusByInternalUserIdParams {
    #[serde(rename = "internalUserId")]
    pub internal_user_id: String,
    #[serde(rename = "status")]
    pub status: (),
    #[serde(rename = "provider")]
    pub provider: (),
}
impl Schema for AuthAuthDeviceUpdateProviderStatusByInternalUserIdParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"internalUserId\":{\"type\":\"string\"},\"status\":{},\"provider\":{}},\"required\":[\"internalUserId\",\"provider\",\"status\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceUpdateProviderStatusByInternalUserIdReturns(bool);
impl Schema for AuthAuthDeviceUpdateProviderStatusByInternalUserIdReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
