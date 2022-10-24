// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionUpdateParamsExtraDetailsParams {
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    #[serde(rename = "tab")]
    pub tab: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionUpdateParams {
    #[serde(rename = "clientKey", deserialize_with = "deserialize_to_string")]
    pub client_key: String,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "extraDetails")]
    pub extra_details: Option<AdminOptionUpdateParamsExtraDetailsParams>,
    #[serde(rename = "value")]
    pub value: Option<String>,
}
impl Schema for AdminOptionUpdateParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"clientKey\":{\"type\":\"string\"},\"description\":{\"type\":\"string\"},\"extraDetails\":{\"type\":\"object\",\"properties\":{\"tab\":{\"type\":\"string\"},\"type\":{\"type\":\"string\"}}},\"value\":{\"type\":\"string\"}},\"required\":[\"clientKey\"]}") . unwrap ()
    }
}
impl Agent for AdminOptionUpdateParams {
    fn topic() -> &'static str {
        "admin_option_update"
    }
    fn method() -> &'static str {
        "option_update"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionUpdateReturns(pub bool);
impl Schema for AdminOptionUpdateReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AdminOptionUpdateReturns {
    fn topic() -> &'static str {
        "admin_option_update"
    }
    fn method() -> &'static str {
        "option_update"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
