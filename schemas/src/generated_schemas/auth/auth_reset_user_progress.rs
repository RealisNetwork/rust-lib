// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthResetUserProgressParams {
    #[serde(rename = "internalUserId")]
    pub internal_user_id: String,
    #[serde(rename = "appId")]
    pub app_id: f64,
}
impl Schema for AuthAuthResetUserProgressParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"internalUserId\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"internalUserId\",\"appId\"]}") . unwrap ()
    }
}
impl Agent for AuthAuthResetUserProgressParams {
    fn topic() -> &'static str {
        "auth_auth_resetUserProgress"
    }
    fn method() -> &'static str {
        "auth_resetUserProgress"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthResetUserProgressReturns(pub bool);
impl Schema for AuthAuthResetUserProgressReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthAuthResetUserProgressReturns {
    fn topic() -> &'static str {
        "auth_auth_resetUserProgress"
    }
    fn method() -> &'static str {
        "auth_resetUserProgress"
    }
    fn agent() -> &'static str {
        "auth"
    }
}