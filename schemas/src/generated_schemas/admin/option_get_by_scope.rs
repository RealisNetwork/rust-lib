// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionGetByScopeParams {
    #[serde(rename = "scope")]
    pub scope: String,
}
impl Schema for AdminOptionGetByScopeParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"scope\":{\"type\":\"string\"}},\"required\":[\"scope\"]}")
    }
}
impl Agent for AdminOptionGetByScopeParams {
    fn topic() -> &'static str {
        "admin_option_getByScope"
    }
    fn method() -> &'static str {
        "option_getByScope"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionGetByScopeReturnsParams {
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "key")]
    pub key: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionGetByScopeReturns(pub Vec<AdminOptionGetByScopeReturnsParams>);
impl Schema for AdminOptionGetByScopeReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"scope\":{\"type\":\"string\"},\"value\":{\"type\":\"string\"},\"key\":{\"type\":\"string\"}},\"required\":[\"scope\",\"key\",\"value\"]}}")
    }
}
impl Agent for AdminOptionGetByScopeReturns {
    fn topic() -> &'static str {
        "admin_option_getByScope"
    }
    fn method() -> &'static str {
        "option_getByScope"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
