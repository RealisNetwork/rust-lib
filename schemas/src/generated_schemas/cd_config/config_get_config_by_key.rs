// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdConfigConfigGetConfigByKeyParams {
    #[serde(rename = "key")]
    pub key: String,
}
impl Schema for CdConfigConfigGetConfigByKeyParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"key\":{\"type\":\"string\"}},\"required\":[\"key\"]}") . unwrap ()
    }
}
impl Agent for CdConfigConfigGetConfigByKeyParams {
    fn topic() -> &'static str {
        "cd-config_config_getConfigByKey"
    }
    fn method() -> &'static str {
        "config_getConfigByKey"
    }
    fn agent() -> &'static str {
        "cd-config"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdConfigConfigGetConfigByKeyReturns(Value);
impl Schema for CdConfigConfigGetConfigByKeyReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"object\",\"properties\":{}}")
    }
}
impl Agent for CdConfigConfigGetConfigByKeyReturns {
    fn topic() -> &'static str {
        "cd-config_config_getConfigByKey"
    }
    fn method() -> &'static str {
        "config_getConfigByKey"
    }
    fn agent() -> &'static str {
        "cd-config"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
