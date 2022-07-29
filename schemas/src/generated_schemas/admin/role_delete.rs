// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminRoleDeleteParams {
    #[serde(rename = "roleName")]
    pub role_name: String,
}
impl Schema for AdminRoleDeleteParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"roleName\":{\"type\":\"string\"}},\"required\":[\"roleName\"]}")
    }
}
impl Agent for AdminRoleDeleteParams {
    fn topic() -> &'static str {
        "admin_role_delete"
    }
    fn method() -> &'static str {
        "role_delete"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminRoleDeleteReturns(pub bool);
impl Schema for AdminRoleDeleteReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AdminRoleDeleteReturns {
    fn topic() -> &'static str {
        "admin_role_delete"
    }
    fn method() -> &'static str {
        "role_delete"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
