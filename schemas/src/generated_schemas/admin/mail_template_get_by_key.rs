// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMailTemplateGetByKeyParams {
    #[serde(rename = "key")]
    pub key: String,
}
impl Schema for AdminMailTemplateGetByKeyParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"key\":{\"type\":\"string\"}},\"required\":[\"key\"]}")
    }
}
impl Agent for AdminMailTemplateGetByKeyParams {
    fn topic() -> &'static str {
        "admin_mailTemplate_getByKey"
    }
    fn method() -> &'static str {
        "mailTemplate_getByKey"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMailTemplateGetByKeyReturns(String);
impl Schema for AdminMailTemplateGetByKeyReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"string\"}")
    }
}
impl Agent for AdminMailTemplateGetByKeyReturns {
    fn topic() -> &'static str {
        "admin_mailTemplate_getByKey"
    }
    fn method() -> &'static str {
        "mailTemplate_getByKey"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
