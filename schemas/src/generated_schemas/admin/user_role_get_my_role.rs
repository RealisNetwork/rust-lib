// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for AdminUserRoleGetMyRoleParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(AdminUserRoleGetMyRoleParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct AdminUserRoleGetMyRoleParams;
impl Schema for AdminUserRoleGetMyRoleParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for AdminUserRoleGetMyRoleParams {
    fn topic() -> &'static str {
        "admin_userRole_getMyRole"
    }
    fn method() -> &'static str {
        "userRole_getMyRole"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserRoleGetMyRoleReturns {
    #[serde(rename = "methods")]
    pub methods: Vec<String>,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
}
impl Schema for AdminUserRoleGetMyRoleReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"methods\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"name\":{\"type\":\"string\"}},\"required\":[\"id\",\"name\",\"methods\"]}")
    }
}
impl Agent for AdminUserRoleGetMyRoleReturns {
    fn topic() -> &'static str {
        "admin_userRole_getMyRole"
    }
    fn method() -> &'static str {
        "userRole_getMyRole"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
