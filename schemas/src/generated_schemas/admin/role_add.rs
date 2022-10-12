// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminRoleAddParams {
    #[serde(rename = "roleName")]
    pub role_name: String,
    #[serde(rename = "permissions")]
    pub permissions: Option<Vec<String>>,
}
impl Schema for AdminRoleAddParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"roleName\":{\"type\":\"string\"},\"permissions\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}}},\"required\":[\"roleName\"]}") . unwrap ()
    }
}
impl Agent for AdminRoleAddParams {
    fn topic() -> &'static str {
        "admin_role_add"
    }
    fn method() -> &'static str {
        "role_add"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminRoleAddReturns(pub bool);
impl Schema for AdminRoleAddReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AdminRoleAddReturns {
    fn topic() -> &'static str {
        "admin_role_add"
    }
    fn method() -> &'static str {
        "role_add"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
