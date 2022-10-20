// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMailTemplateGetByKeyParams {
    #[serde(rename = "key", deserialize_with = "deserialize_to_string")]
    pub key: String,
}
impl Schema for AdminMailTemplateGetByKeyParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"key\":{\"type\":\"string\"}},\"required\":[\"key\"]}") . unwrap ()
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMailTemplateGetByKeyReturns(pub String);
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
