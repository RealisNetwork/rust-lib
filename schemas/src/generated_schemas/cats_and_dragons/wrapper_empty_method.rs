// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperEmptyMethodParams {
    #[serde(rename = "accessToken", deserialize_with = "deserialize_to_string")]
    pub access_token: String,
}
impl Schema for CatsAndDragonsWrapperEmptyMethodParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"accessToken\":{\"type\":\"string\"}},\"required\":[\"accessToken\"]}") . unwrap ()
    }
}
impl Agent for CatsAndDragonsWrapperEmptyMethodParams {
    fn topic() -> &'static str {
        "catsAndDragons_wrapper_emptyMethod"
    }
    fn method() -> &'static str {
        "wrapper_emptyMethod"
    }
    fn agent() -> &'static str {
        "catsAndDragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperEmptyMethodReturns(pub Value);
impl Schema for CatsAndDragonsWrapperEmptyMethodReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"object\",\"properties\":{}}")
    }
}
impl Agent for CatsAndDragonsWrapperEmptyMethodReturns {
    fn topic() -> &'static str {
        "catsAndDragons_wrapper_emptyMethod"
    }
    fn method() -> &'static str {
        "wrapper_emptyMethod"
    }
    fn agent() -> &'static str {
        "catsAndDragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
