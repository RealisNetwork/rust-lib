// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAdminDeleteUserRoleParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for AuthAdminDeleteUserRoleParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
impl Agent for AuthAdminDeleteUserRoleParams {
    fn topic() -> &'static str {
        "auth_admin_deleteUserRole"
    }
    fn method() -> &'static str {
        "admin_deleteUserRole"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAdminDeleteUserRoleReturns(bool);
impl Schema for AuthAdminDeleteUserRoleReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthAdminDeleteUserRoleReturns {
    fn topic() -> &'static str {
        "auth_admin_deleteUserRole"
    }
    fn method() -> &'static str {
        "admin_deleteUserRole"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
