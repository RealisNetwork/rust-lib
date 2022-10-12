// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminPermissionUpdatePermissionsParams {
    #[serde(rename = "permissions")]
    pub permissions: Vec<String>,
    #[serde(rename = "roleName")]
    pub role_name: String,
}
impl Schema for AdminPermissionUpdatePermissionsParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"permissions\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"roleName\":{\"type\":\"string\"}},\"required\":[\"roleName\",\"permissions\"]}") . unwrap ()
    }
}
impl Agent for AdminPermissionUpdatePermissionsParams {
    fn topic() -> &'static str {
        "admin_permission_updatePermissions"
    }
    fn method() -> &'static str {
        "permission_updatePermissions"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminPermissionUpdatePermissionsReturns(pub bool);
impl Schema for AdminPermissionUpdatePermissionsReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AdminPermissionUpdatePermissionsReturns {
    fn topic() -> &'static str {
        "admin_permission_updatePermissions"
    }
    fn method() -> &'static str {
        "permission_updatePermissions"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
