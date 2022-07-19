// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMailTemplateCreateParams {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "mailTemplate")]
    pub mail_template: String,
    #[serde(rename = "name")]
    pub name: String,
}
impl Schema for AdminMailTemplateCreateParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"key\":{\"type\":\"string\"},\"mailTemplate\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"key\",\"name\",\"mailTemplate\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMailTemplateCreateReturns(bool);
impl Schema for AdminMailTemplateCreateReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}