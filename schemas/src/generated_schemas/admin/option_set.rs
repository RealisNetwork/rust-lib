// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionSetParamsExtraDetailsParams {
    #[serde(rename = "tab")]
    pub tab: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionSetParams {
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "clientKey")]
    pub client_key: String,
    #[serde(rename = "extraDetails")]
    pub extra_details: Option<AdminOptionSetParamsExtraDetailsParams>,
    #[serde(rename = "value")]
    pub value: String,
}
impl Schema for AdminOptionSetParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"description\":{\"type\":\"string\"},\"clientKey\":{\"type\":\"string\"},\"extraDetails\":{\"type\":\"object\",\"properties\":{\"tab\":{\"type\":\"string\"},\"type\":{\"type\":\"string\"}}},\"value\":{\"type\":\"string\"}},\"required\":[\"clientKey\",\"value\"]}") . unwrap ()
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
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
}
impl Schema for AdminOptionSetReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"key\":{\"type\":\"string\"},\"value\":{\"type\":\"string\"}},\"required\":[\"key\",\"value\"]}")
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
