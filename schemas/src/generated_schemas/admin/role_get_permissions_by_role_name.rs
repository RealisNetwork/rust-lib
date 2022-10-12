// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminRoleGetPermissionsByRoleNameParams {
    #[serde(rename = "roleName")]
    pub role_name: String,
}
impl Schema for AdminRoleGetPermissionsByRoleNameParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"roleName\":{\"type\":\"string\"}},\"required\":[\"roleName\"]}") . unwrap ()
    }
}
impl Agent for AdminRoleGetPermissionsByRoleNameParams {
    fn topic() -> &'static str {
        "admin_role_getPermissionsByRoleName"
    }
    fn method() -> &'static str {
        "role_getPermissionsByRoleName"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminRoleGetPermissionsByRoleNameReturns(pub Vec<String>);
impl Schema for AdminRoleGetPermissionsByRoleNameReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"array\",\"items\":{\"type\":\"string\"}}")
    }
}
impl Agent for AdminRoleGetPermissionsByRoleNameReturns {
    fn topic() -> &'static str {
        "admin_role_getPermissionsByRoleName"
    }
    fn method() -> &'static str {
        "role_getPermissionsByRoleName"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
