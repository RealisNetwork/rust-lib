// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceUpdateProviderStatusByProviderIdParams {
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "status")]
    pub status: (),
}
impl Schema for AuthAuthDeviceUpdateProviderStatusByProviderIdParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"providerId\":{\"type\":\"string\"},\"status\":{}},\"required\":[\"providerId\",\"status\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceUpdateProviderStatusByProviderIdReturns(bool);
impl Schema for AuthAuthDeviceUpdateProviderStatusByProviderIdReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}