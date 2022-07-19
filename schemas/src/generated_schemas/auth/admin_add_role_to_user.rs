// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAdminAddRoleToUserParams {
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for AuthAdminAddRoleToUserParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"role\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"role\"]}")
    }
}
impl Agent for AuthAdminAddRoleToUserParams {
    fn topic() -> &'static str {
        "auth_admin_addRoleToUser"
    }
    fn method() -> &'static str {
        "admin_addRoleToUser"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAdminAddRoleToUserReturns(bool);
impl Schema for AuthAdminAddRoleToUserReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthAdminAddRoleToUserReturns {
    fn topic() -> &'static str {
        "auth_admin_addRoleToUser"
    }
    fn method() -> &'static str {
        "admin_addRoleToUser"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
