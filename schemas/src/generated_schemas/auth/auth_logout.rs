// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthLogoutParams {
    #[serde(rename = "appId")]
    pub app_id: Option<i32>,
}
impl Schema for AuthAuthLogoutParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"appId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}}}") . unwrap ()
    }
}
impl Agent for AuthAuthLogoutParams {
    fn topic() -> &'static str {
        "auth_auth_logout"
    }
    fn method() -> &'static str {
        "auth_logout"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthLogoutReturns(pub bool);
impl Schema for AuthAuthLogoutReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthAuthLogoutReturns {
    fn topic() -> &'static str {
        "auth_auth_logout"
    }
    fn method() -> &'static str {
        "auth_logout"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
