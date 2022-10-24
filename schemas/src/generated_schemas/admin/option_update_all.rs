// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionUpdateAllParamsClientKeysParamsParamsExtraDetailsParams {
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    #[serde(rename = "tab")]
    pub tab: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionUpdateAllParamsClientKeysParamsParams {
    #[serde(rename = "value", deserialize_with = "deserialize_to_string")]
    pub value: String,
    #[serde(rename = "scope", deserialize_with = "deserialize_to_string")]
    pub scope: String,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "extraDetails")]
    pub extra_details: Option<AdminOptionUpdateAllParamsClientKeysParamsParamsExtraDetailsParams>,
    #[serde(rename = "key", deserialize_with = "deserialize_to_string")]
    pub key: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionUpdateAllParams {
    #[serde(rename = "clientKeys")]
    pub client_keys: Vec<AdminOptionUpdateAllParamsClientKeysParamsParams>,
}
impl Schema for AdminOptionUpdateAllParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"clientKeys\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"value\":{\"type\":\"string\"},\"scope\":{\"type\":\"string\"},\"description\":{\"type\":\"string\"},\"extraDetails\":{\"type\":\"object\",\"properties\":{\"type\":{\"type\":\"string\"},\"tab\":{\"type\":\"string\"}}},\"key\":{\"type\":\"string\"}},\"required\":[\"scope\",\"key\",\"value\"]}}},\"required\":[\"clientKeys\"]}") . unwrap ()
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
