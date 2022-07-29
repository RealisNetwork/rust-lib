// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthConfirmEmailByHashParams {
    #[serde(rename = "hash")]
    pub hash: String,
}
impl Schema for AuthAuthConfirmEmailByHashParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"hash\":{\"type\":\"string\"}},\"required\":[\"hash\"]}")
    }
}
impl Agent for AuthAuthConfirmEmailByHashParams {
    fn topic() -> &'static str {
        "auth_auth_confirmEmailByHash"
    }
    fn method() -> &'static str {
        "auth_confirmEmailByHash"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthConfirmEmailByHashReturns(pub String);
impl Schema for AuthAuthConfirmEmailByHashReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"string\"}")
    }
}
impl Agent for AuthAuthConfirmEmailByHashReturns {
    fn topic() -> &'static str {
        "auth_auth_confirmEmailByHash"
    }
    fn method() -> &'static str {
        "auth_confirmEmailByHash"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
