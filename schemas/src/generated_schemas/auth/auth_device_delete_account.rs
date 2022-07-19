// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceDeleteAccountParams {
    #[serde(rename = "appId")]
    pub app_id: i32,
}
impl Schema for AuthAuthDeviceDeleteAccountParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"appId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"appId\"]}")
    }
}
impl Agent for AuthAuthDeviceDeleteAccountParams {
    fn topic() -> &'static str {
        "auth_authDevice_deleteAccount"
    }
    fn method() -> &'static str {
        "authDevice_deleteAccount"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceDeleteAccountReturns(bool);
impl Schema for AuthAuthDeviceDeleteAccountReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthAuthDeviceDeleteAccountReturns {
    fn topic() -> &'static str {
        "auth_authDevice_deleteAccount"
    }
    fn method() -> &'static str {
        "authDevice_deleteAccount"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
