// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionGetParams {
    #[serde(rename = "clientKey")]
    pub client_key: String,
}
impl Schema for AdminOptionGetParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"clientKey\":{\"type\":\"string\"}},\"required\":[\"clientKey\"]}") . unwrap ()
    }
}
impl Agent for AdminOptionGetParams {
    fn topic() -> &'static str {
        "admin_option_get"
    }
    fn method() -> &'static str {
        "option_get"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionGetReturns {
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "key")]
    pub key: String,
}
impl Schema for AdminOptionGetReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"value\":{\"type\":\"string\"},\"scope\":{\"type\":\"string\"},\"key\":{\"type\":\"string\"}},\"required\":[\"scope\",\"key\",\"value\"]}")
    }
}
impl Agent for AdminOptionGetReturns {
    fn topic() -> &'static str {
        "admin_option_get"
    }
    fn method() -> &'static str {
        "option_get"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
