// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceIsNewClientParams {
    #[serde(rename = "appId")]
    pub app_id: f64,
    #[serde(rename = "internalUserId", deserialize_with = "deserialize_to_string")]
    pub internal_user_id: String,
}
impl Schema for AuthAuthDeviceIsNewClientParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"internalUserId\":{\"type\":\"string\"}},\"required\":[\"internalUserId\",\"appId\"]}") . unwrap ()
    }
}
impl Agent for AuthAuthDeviceIsNewClientParams {
    fn topic() -> &'static str {
        "auth_authDevice_isNewClient"
    }
    fn method() -> &'static str {
        "authDevice_isNewClient"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceIsNewClientReturns(pub bool);
impl Schema for AuthAuthDeviceIsNewClientReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthAuthDeviceIsNewClientReturns {
    fn topic() -> &'static str {
        "auth_authDevice_isNewClient"
    }
    fn method() -> &'static str {
        "authDevice_isNewClient"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
