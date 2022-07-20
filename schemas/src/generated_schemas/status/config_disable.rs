// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusConfigDisableParams {
    #[serde(rename = "id")]
    pub id: i64,
}
impl Schema for StatusConfigDisableParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}")
    }
}
impl Agent for StatusConfigDisableParams {
    fn topic() -> &'static str {
        "status_config_disable"
    }
    fn method() -> &'static str {
        "config_disable"
    }
    fn agent() -> &'static str {
        "status"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusConfigDisableReturns(bool);
impl Schema for StatusConfigDisableReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for StatusConfigDisableReturns {
    fn topic() -> &'static str {
        "status_config_disable"
    }
    fn method() -> &'static str {
        "config_disable"
    }
    fn agent() -> &'static str {
        "status"
    }
}
