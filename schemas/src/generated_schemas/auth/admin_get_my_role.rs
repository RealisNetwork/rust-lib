// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAdminGetMyRoleParams {}
impl Schema for AuthAdminGetMyRoleParams {
    fn schema() -> Value {
        serde_json::from_str("{\"type\":\"object\",\"properties\":{}}").unwrap()
    }
}
impl Agent for AuthAdminGetMyRoleParams {
    fn topic() -> &'static str {
        "auth_admin_getMyRole"
    }
    fn method() -> &'static str {
        "admin_getMyRole"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAdminGetMyRoleReturns {
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "pages")]
    pub pages: Vec<String>,
    #[serde(rename = "methods")]
    pub methods: Vec<String>,
}
impl Schema for AuthAdminGetMyRoleReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"name\":{\"type\":\"string\"},\"pages\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"methods\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}}},\"required\":[\"id\",\"name\",\"methods\",\"pages\"]}")
    }
}
impl Agent for AuthAdminGetMyRoleReturns {
    fn topic() -> &'static str {
        "auth_admin_getMyRole"
    }
    fn method() -> &'static str {
        "admin_getMyRole"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
