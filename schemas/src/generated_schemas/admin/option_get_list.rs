// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for AdminOptionGetListParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(AdminOptionGetListParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct AdminOptionGetListParams;
impl Schema for AdminOptionGetListParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for AdminOptionGetListParams {
    fn topic() -> &'static str {
        "admin_option_getList"
    }
    fn method() -> &'static str {
        "option_getList"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionGetListReturnsParams {
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "key")]
    pub key: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionGetListReturns(pub Vec<AdminOptionGetListReturnsParams>);
impl Schema for AdminOptionGetListReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"value\":{\"type\":\"string\"},\"scope\":{\"type\":\"string\"},\"key\":{\"type\":\"string\"}},\"required\":[\"scope\",\"key\",\"value\"]}}")
    }
}
impl Agent for AdminOptionGetListReturns {
    fn topic() -> &'static str {
        "admin_option_getList"
    }
    fn method() -> &'static str {
        "option_getList"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
