// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminPermissionDeleteParams {
    #[serde(rename = "roleName")]
    pub role_name: String,
    #[serde(rename = "permissionName")]
    pub permission_name: Vec<String>,
}
impl Schema for AdminPermissionDeleteParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"roleName\":{\"type\":\"string\"},\"permissionName\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}}},\"required\":[\"permissionName\",\"roleName\"]}") . unwrap ()
    }
}
impl Agent for AdminPermissionDeleteParams {
    fn topic() -> &'static str {
        "admin_permission_delete"
    }
    fn method() -> &'static str {
        "permission_delete"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminPermissionDeleteReturns(pub bool);
impl Schema for AdminPermissionDeleteReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AdminPermissionDeleteReturns {
    fn topic() -> &'static str {
        "admin_permission_delete"
    }
    fn method() -> &'static str {
        "permission_delete"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
