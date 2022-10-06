// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for AdminUserRoleGetAllParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(AdminUserRoleGetAllParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct AdminUserRoleGetAllParams;
impl Schema for AdminUserRoleGetAllParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for AdminUserRoleGetAllParams {
    fn topic() -> &'static str {
        "admin_userRole_getAll"
    }
    fn method() -> &'static str {
        "userRole_getAll"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserRoleGetAllReturnsParamsRoleParams {
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "name")]
    pub name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserRoleGetAllReturnsParams {
    #[serde(rename = "role")]
    pub role: AdminUserRoleGetAllReturnsParamsRoleParams,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserRoleGetAllReturns(pub Vec<AdminUserRoleGetAllReturnsParams>);
impl Schema for AdminUserRoleGetAllReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"role\":{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"name\":{\"type\":\"string\"}},\"required\":[\"id\",\"name\"]},\"isActive\":{\"type\":\"boolean\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"id\",\"userId\",\"isActive\",\"role\"]}}")
    }
}
impl Agent for AdminUserRoleGetAllReturns {
    fn topic() -> &'static str {
        "admin_userRole_getAll"
    }
    fn method() -> &'static str {
        "userRole_getAll"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
