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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionGetByScopeReturnsParams {
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "key")]
    pub key: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionGetByScopeReturns(Vec<AdminOptionGetByScopeReturnsParams>);
impl Schema for AdminOptionGetByScopeReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"value\":{\"type\":\"string\"},\"scope\":{\"type\":\"string\"},\"key\":{\"type\":\"string\"}},\"required\":[\"scope\",\"key\",\"value\"]}}")
    }
}