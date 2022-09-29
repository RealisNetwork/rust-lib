// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthChangeMyEmailParams {
    #[serde(rename = "hash")]
    pub hash: String,
}
impl Schema for AuthAuthChangeMyEmailParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"hash\":{\"type\":\"string\"}},\"required\":[\"hash\"]}") . unwrap ()
    }
}
impl Agent for AuthAuthChangeMyEmailParams {
    fn topic() -> &'static str {
        "auth_auth_changeMyEmail"
    }
    fn method() -> &'static str {
        "auth_changeMyEmail"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthChangeMyEmailReturns(pub bool);
impl Schema for AuthAuthChangeMyEmailReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthAuthChangeMyEmailReturns {
    fn topic() -> &'static str {
        "auth_auth_changeMyEmail"
    }
    fn method() -> &'static str {
        "auth_changeMyEmail"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
