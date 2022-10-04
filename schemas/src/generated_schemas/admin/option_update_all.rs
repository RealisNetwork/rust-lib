// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionUpdateAllParamsClientKeysParamsParamsExtraDetailsParams {
    #[serde(rename = "tab")]
    pub tab: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionUpdateAllParamsClientKeysParamsParams {
    #[serde(rename = "extraDetails")]
    pub extra_details: Option<AdminOptionUpdateAllParamsClientKeysParamsParamsExtraDetailsParams>,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "description")]
    pub description: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionUpdateAllParams {
    #[serde(rename = "clientKeys")]
    pub client_keys: Vec<AdminOptionUpdateAllParamsClientKeysParamsParams>,
}
impl Schema for AdminOptionUpdateAllParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"clientKeys\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"extraDetails\":{\"type\":\"object\",\"properties\":{\"tab\":{\"type\":\"string\"},\"type\":{\"type\":\"string\"}}},\"value\":{\"type\":\"string\"},\"key\":{\"type\":\"string\"},\"scope\":{\"type\":\"string\"},\"description\":{\"type\":\"string\"}},\"required\":[\"scope\",\"key\",\"value\"]}}},\"required\":[\"clientKeys\"]}") . unwrap ()
    }
}
impl Agent for AdminOptionUpdateAllParams {
    fn topic() -> &'static str {
        "admin_option_updateAll"
    }
    fn method() -> &'static str {
        "option_updateAll"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionUpdateAllReturns(pub bool);
impl Schema for AdminOptionUpdateAllReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AdminOptionUpdateAllReturns {
    fn topic() -> &'static str {
        "admin_option_updateAll"
    }
    fn method() -> &'static str {
        "option_updateAll"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
