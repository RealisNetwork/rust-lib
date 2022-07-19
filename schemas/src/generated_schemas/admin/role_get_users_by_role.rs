// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminRoleGetUsersByRoleParams {
    #[serde(rename = "roleName")]
    pub role_name: String,
}
impl Schema for AdminRoleGetUsersByRoleParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"roleName\":{\"type\":\"string\"}},\"required\":[\"roleName\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminRoleGetUsersByRoleReturns(Vec<String>);
impl Schema for AdminRoleGetUsersByRoleReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"array\",\"items\":{\"type\":\"string\"}}")
    }
}
