// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminRoleGetAllRolesParams(Value);
impl Schema for AdminRoleGetAllRolesParams {
    fn schema() -> Value {
        serde_json::from_str("{\"type\":\"object\",\"properties\":{}}").unwrap()
    }
}
impl Agent for AdminRoleGetAllRolesParams {
    fn topic() -> &'static str {
        "admin_role_getAllRoles"
    }
    fn method() -> &'static str {
        "role_getAllRoles"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminRoleGetAllRolesReturns(pub Vec<String>);
impl Schema for AdminRoleGetAllRolesReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"array\",\"items\":{\"type\":\"string\"}}")
    }
}
impl Agent for AdminRoleGetAllRolesReturns {
    fn topic() -> &'static str {
        "admin_role_getAllRoles"
    }
    fn method() -> &'static str {
        "role_getAllRoles"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
