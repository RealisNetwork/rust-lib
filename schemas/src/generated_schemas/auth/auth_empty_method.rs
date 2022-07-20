// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for AuthAuthEmptyMethodParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(AuthAuthEmptyMethodParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct AuthAuthEmptyMethodParams;
impl Schema for AuthAuthEmptyMethodParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for AuthAuthEmptyMethodParams {
    fn topic() -> &'static str {
        "auth_auth_emptyMethod"
    }
    fn method() -> &'static str {
        "auth_emptyMethod"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthEmptyMethodReturns(bool);
impl Schema for AuthAuthEmptyMethodReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthAuthEmptyMethodReturns {
    fn topic() -> &'static str {
        "auth_auth_emptyMethod"
    }
    fn method() -> &'static str {
        "auth_emptyMethod"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
