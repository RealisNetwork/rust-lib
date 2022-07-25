// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdConfigConfigAddConfigParamsDataParams {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdConfigConfigAddConfigParams {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "data")]
    pub data: CdConfigConfigAddConfigParamsDataParams,
}
impl Schema for CdConfigConfigAddConfigParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"key\":{\"type\":\"string\"},\"data\":{\"type\":\"object\",\"properties\":{},\"required\":null}},\"required\":[\"key\",\"data\"]}")
    }
}
impl Agent for CdConfigConfigAddConfigParams {
    fn topic() -> &'static str {
        "cd-config_config_addConfig"
    }
    fn method() -> &'static str {
        "config_addConfig"
    }
    fn agent() -> &'static str {
        "cd-config"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdConfigConfigAddConfigReturns(bool);
impl Schema for CdConfigConfigAddConfigReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for CdConfigConfigAddConfigReturns {
    fn topic() -> &'static str {
        "cd-config_config_addConfig"
    }
    fn method() -> &'static str {
        "config_addConfig"
    }
    fn agent() -> &'static str {
        "cd-config"
    }
}
