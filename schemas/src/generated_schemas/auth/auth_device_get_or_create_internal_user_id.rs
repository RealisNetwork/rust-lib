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
    #[serde(rename = "appId")]
    pub app_id: Option<i32>,
    #[serde(rename = "provider")]
    pub provider: (),
}
impl Schema for AuthAuthDeviceGetOrCreateInternalUserIdParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"providerId\":{\"type\":\"string\"},\"email\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"provider\":{}},\"required\":[\"providerId\",\"provider\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceGetOrCreateInternalUserIdReturns(String);
impl Schema for AuthAuthDeviceGetOrCreateInternalUserIdReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"string\"}")
    }
}