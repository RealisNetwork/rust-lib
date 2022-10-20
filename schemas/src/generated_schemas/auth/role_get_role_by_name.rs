// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRoleGetRoleByNameParams {
    #[serde(rename = "roleName", deserialize_with = "deserialize_to_string")]
    pub role_name: String,
}
impl Schema for AuthRoleGetRoleByNameParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"roleName\":{\"type\":\"string\"}},\"required\":[\"roleName\"]}") . unwrap ()
    }
}
impl Agent for AuthRoleGetRoleByNameParams {
    fn topic() -> &'static str {
        "auth_role_getRoleByName"
    }
    fn method() -> &'static str {
        "role_getRoleByName"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRoleGetRoleByNameReturns {
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "methods")]
    pub methods: Vec<String>,
    #[serde(rename = "pages")]
    pub pages: Vec<String>,
    #[serde(rename = "name", deserialize_with = "deserialize_to_string")]
    pub name: String,
}
impl Schema for AuthRoleGetRoleByNameReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"methods\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"pages\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"name\":{\"type\":\"string\"}},\"required\":[\"id\",\"name\",\"methods\",\"pages\"]}")
    }
}
impl Agent for AuthRoleGetRoleByNameReturns {
    fn topic() -> &'static str {
        "auth_role_getRoleByName"
    }
    fn method() -> &'static str {
        "role_getRoleByName"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
