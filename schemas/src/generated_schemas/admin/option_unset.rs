// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionUnsetParams {
    #[serde(rename = "clientKey")]
    pub client_key: String,
}
impl Schema for AdminOptionUnsetParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"clientKey\":{\"type\":\"string\"}},\"required\":[\"clientKey\"]}") . unwrap ()
    }
}
impl Agent for AdminOptionUnsetParams {
    fn topic() -> &'static str {
        "admin_option_unset"
    }
    fn method() -> &'static str {
        "option_unset"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionUnsetReturns(pub bool);
impl Schema for AdminOptionUnsetReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AdminOptionUnsetReturns {
    fn topic() -> &'static str {
        "admin_option_unset"
    }
    fn method() -> &'static str {
        "option_unset"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
