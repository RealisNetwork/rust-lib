// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdUserSpinEndpointsSpinParams {
    #[serde(rename = "Type")]
    pub r#type: String,
}
impl Schema for CdUserSpinEndpointsSpinParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"Type\":{\"type\":\"string\"}},\"required\":[\"Type\"]}")
    }
}
impl Agent for CdUserSpinEndpointsSpinParams {
    fn topic() -> &'static str {
        "cd-user_spinEndpoints_spin"
    }
    fn method() -> &'static str {
        "spinEndpoints_spin"
    }
    fn agent() -> &'static str {
        "cd-user"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdUserSpinEndpointsSpinReturns {
    #[serde(rename = "Key")]
    pub key: String,
}
impl Schema for CdUserSpinEndpointsSpinReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"Key\":{\"type\":\"string\"}},\"required\":[\"Key\"]}")
    }
}
impl Agent for CdUserSpinEndpointsSpinReturns {
    fn topic() -> &'static str {
        "cd-user_spinEndpoints_spin"
    }
    fn method() -> &'static str {
        "spinEndpoints_spin"
    }
    fn agent() -> &'static str {
        "cd-user"
    }
}
