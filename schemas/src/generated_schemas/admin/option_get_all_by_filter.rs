// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionGetAllByFilterParams {
    #[serde(rename = "tab")]
    pub tab: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
impl Schema for AdminOptionGetAllByFilterParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"tab\":{\"type\":\"string\"},\"type\":{\"type\":\"string\"}}}") . unwrap ()
    }
}
impl Agent for AdminOptionGetAllByFilterParams {
    fn topic() -> &'static str {
        "admin_option_getAllByFilter"
    }
    fn method() -> &'static str {
        "option_getAllByFilter"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionGetAllByFilterReturnsParamsExtraDetailsParams {
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    #[serde(rename = "tab")]
    pub tab: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionGetAllByFilterReturnsParams {
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "previousValue")]
    pub previous_value: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "extraDetails")]
    pub extra_details: AdminOptionGetAllByFilterReturnsParamsExtraDetailsParams,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionGetAllByFilterReturns(pub Vec<AdminOptionGetAllByFilterReturnsParams>);
impl Schema for AdminOptionGetAllByFilterReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"value\":{\"type\":\"string\"},\"scope\":{\"type\":\"string\"},\"key\":{\"type\":\"string\"},\"previousValue\":{\"type\":\"string\"},\"description\":{\"type\":\"string\"},\"extraDetails\":{\"type\":\"object\",\"properties\":{\"type\":{\"type\":\"string\"},\"tab\":{\"type\":\"string\"}}}},\"required\":[\"scope\",\"key\",\"value\",\"previousValue\",\"description\",\"extraDetails\"]}}")
    }
}
impl Agent for AdminOptionGetAllByFilterReturns {
    fn topic() -> &'static str {
        "admin_option_getAllByFilter"
    }
    fn method() -> &'static str {
        "option_getAllByFilter"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
