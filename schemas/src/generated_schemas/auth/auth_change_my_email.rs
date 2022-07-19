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
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"hash\":{\"type\":\"string\"}},\"required\":[\"hash\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthChangeMyEmailReturns(bool);
impl Schema for AuthAuthChangeMyEmailReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}