// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for AdminUserRoleIsAdminParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(AdminUserRoleIsAdminParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct AdminUserRoleIsAdminParams;
impl Schema for AdminUserRoleIsAdminParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for AdminUserRoleIsAdminParams {
    fn topic() -> &'static str {
        "admin_userRole_isAdmin"
    }
    fn method() -> &'static str {
        "userRole_isAdmin"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserRoleIsAdminReturns {
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
}
impl Schema for AdminUserRoleIsAdminReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"isAdmin\":{\"type\":\"boolean\"}},\"required\":[\"isAdmin\"]}")
    }
}
impl Agent for AdminUserRoleIsAdminReturns {
    fn topic() -> &'static str {
        "admin_userRole_isAdmin"
    }
    fn method() -> &'static str {
        "userRole_isAdmin"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
