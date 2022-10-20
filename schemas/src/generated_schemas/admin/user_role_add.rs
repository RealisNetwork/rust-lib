// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserRoleAddParams {
    #[serde(rename = "internalUserId", deserialize_with = "deserialize_to_string")]
    pub internal_user_id: String,
    #[serde(rename = "roleName", deserialize_with = "deserialize_to_string")]
    pub role_name: String,
}
impl Schema for AdminUserRoleAddParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"internalUserId\":{\"type\":\"string\"},\"roleName\":{\"type\":\"string\"}},\"required\":[\"roleName\",\"internalUserId\"]}") . unwrap ()
    }
}
impl Agent for AdminUserRoleAddParams {
    fn topic() -> &'static str {
        "admin_userRole_add"
    }
    fn method() -> &'static str {
        "userRole_add"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserRoleAddReturns(pub bool);
impl Schema for AdminUserRoleAddReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AdminUserRoleAddReturns {
    fn topic() -> &'static str {
        "admin_userRole_add"
    }
    fn method() -> &'static str {
        "userRole_add"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
