// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMailTemplateCreateParams {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "mailTemplate")]
    pub mail_template: String,
}
impl Schema for AdminMailTemplateCreateParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"name\":{\"type\":\"string\"},\"key\":{\"type\":\"string\"},\"mailTemplate\":{\"type\":\"string\"}},\"required\":[\"key\",\"name\",\"mailTemplate\"]}") . unwrap ()
    }
}
impl Agent for AdminMailTemplateCreateParams {
    fn topic() -> &'static str {
        "admin_mailTemplate_create"
    }
    fn method() -> &'static str {
        "mailTemplate_create"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMailTemplateCreateReturns(pub bool);
impl Schema for AdminMailTemplateCreateReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AdminMailTemplateCreateReturns {
    fn topic() -> &'static str {
        "admin_mailTemplate_create"
    }
    fn method() -> &'static str {
        "mailTemplate_create"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
