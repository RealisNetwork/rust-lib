// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionSetParamsExtraDetailsParams {
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    #[serde(rename = "tab")]
    pub tab: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionSetParams {
    #[serde(rename = "value", deserialize_with = "deserialize_to_string")]
    pub value: String,
    #[serde(rename = "extraDetails")]
    pub extra_details: Option<AdminOptionSetParamsExtraDetailsParams>,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "clientKey", deserialize_with = "deserialize_to_string")]
    pub client_key: String,
}
impl Schema for AdminOptionSetParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"value\":{\"type\":\"string\"},\"extraDetails\":{\"type\":\"object\",\"properties\":{\"type\":{\"type\":\"string\"},\"tab\":{\"type\":\"string\"}}},\"description\":{\"type\":\"string\"},\"clientKey\":{\"type\":\"string\"}},\"required\":[\"clientKey\",\"value\"]}") . unwrap ()
    }
}
impl Agent for AdminOptionSetParams {
    fn topic() -> &'static str {
        "admin_option_set"
    }
    fn method() -> &'static str {
        "option_set"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionSetReturns {
    #[serde(rename = "value", deserialize_with = "deserialize_to_string")]
    pub value: String,
    #[serde(rename = "key", deserialize_with = "deserialize_to_string")]
    pub key: String,
}
impl Schema for AdminOptionSetReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"value\":{\"type\":\"string\"},\"key\":{\"type\":\"string\"}},\"required\":[\"key\",\"value\"]}")
    }
}
impl Agent for AdminOptionSetReturns {
    fn topic() -> &'static str {
        "admin_option_set"
    }
    fn method() -> &'static str {
        "option_set"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
