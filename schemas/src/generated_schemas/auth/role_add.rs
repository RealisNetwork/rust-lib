// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRoleAddParams {
    #[serde(rename = "methods")]
    pub methods: Vec<String>,
    #[serde(rename = "roleName")]
    pub role_name: String,
    #[serde(rename = "pages")]
    pub pages: Vec<String>,
}
impl Schema for AuthRoleAddParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"methods\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"roleName\":{\"type\":\"string\"},\"pages\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}}},\"required\":[\"roleName\",\"methods\",\"pages\"]}") . unwrap ()
    }
}
impl Agent for AuthRoleAddParams {
    fn topic() -> &'static str {
        "auth_role_add"
    }
    fn method() -> &'static str {
        "role_add"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRoleAddReturns(pub bool);
impl Schema for AuthRoleAddReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthRoleAddReturns {
    fn topic() -> &'static str {
        "auth_role_add"
    }
    fn method() -> &'static str {
        "role_add"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
