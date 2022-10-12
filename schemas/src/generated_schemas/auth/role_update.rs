// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRoleUpdateParams {
    #[serde(rename = "methods")]
    pub methods: Vec<String>,
    #[serde(rename = "roleId")]
    pub role_id: f64,
    #[serde(rename = "pages")]
    pub pages: Vec<String>,
}
impl Schema for AuthRoleUpdateParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"methods\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"roleId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"pages\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}}},\"required\":[\"roleId\",\"methods\",\"pages\"]}") . unwrap ()
    }
}
impl Agent for AuthRoleUpdateParams {
    fn topic() -> &'static str {
        "auth_role_update"
    }
    fn method() -> &'static str {
        "role_update"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRoleUpdateReturns(pub bool);
impl Schema for AuthRoleUpdateReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthRoleUpdateReturns {
    fn topic() -> &'static str {
        "auth_role_update"
    }
    fn method() -> &'static str {
        "role_update"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
