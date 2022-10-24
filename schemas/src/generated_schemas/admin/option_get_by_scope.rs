// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionGetByScopeParams {
    #[serde(rename = "scope", deserialize_with = "deserialize_to_string")]
    pub scope: String,
}
impl Schema for AdminOptionGetByScopeParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"scope\":{\"type\":\"string\"}},\"required\":[\"scope\"]}") . unwrap ()
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionGetByScopeReturnsParams {
    #[serde(rename = "scope", deserialize_with = "deserialize_to_string")]
    pub scope: String,
    #[serde(rename = "key", deserialize_with = "deserialize_to_string")]
    pub key: String,
    #[serde(rename = "value", deserialize_with = "deserialize_to_string")]
    pub value: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionGetByScopeReturns(pub Vec<AdminOptionGetByScopeReturnsParams>);
impl Schema for AdminOptionGetByScopeReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"scope\":{\"type\":\"string\"},\"key\":{\"type\":\"string\"},\"value\":{\"type\":\"string\"}},\"required\":[\"scope\",\"key\",\"value\"]}}")
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
