// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionUpdateParamsExtraDetailsParams {
    #[serde(rename = "tab")]
    pub tab: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionUpdateParams {
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "clientKey")]
    pub client_key: String,
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(rename = "extraDetails")]
    pub extra_details: Option<AdminOptionUpdateParamsExtraDetailsParams>,
}
impl Schema for AdminOptionUpdateParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"description\":{\"type\":\"string\"},\"clientKey\":{\"type\":\"string\"},\"value\":{\"type\":\"string\"},\"extraDetails\":{\"type\":\"object\",\"properties\":{\"tab\":{\"type\":\"string\"},\"type\":{\"type\":\"string\"}}}},\"required\":[\"clientKey\"]}") . unwrap ()
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
}
