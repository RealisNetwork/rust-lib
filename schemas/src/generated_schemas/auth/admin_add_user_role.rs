// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAdminAddUserRoleParams {
    #[serde(rename = "roleName")]
    pub role_name: String,
    #[serde(rename = "internalUserId")]
    pub internal_user_id: String,
}
impl Schema for AuthAdminAddUserRoleParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"roleName\":{\"type\":\"string\"},\"internalUserId\":{\"type\":\"string\"}},\"required\":[\"roleName\",\"internalUserId\"]}") . unwrap ()
    }
}
impl Agent for AuthAdminAddUserRoleParams {
    fn topic() -> &'static str {
        "auth_admin_addUserRole"
    }
    fn method() -> &'static str {
        "admin_addUserRole"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAdminAddUserRoleReturns(pub bool);
impl Schema for AuthAdminAddUserRoleReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthAdminAddUserRoleReturns {
    fn topic() -> &'static str {
        "auth_admin_addUserRole"
    }
    fn method() -> &'static str {
        "admin_addUserRole"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
