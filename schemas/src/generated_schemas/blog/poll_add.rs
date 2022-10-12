// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPollAddParamsAnswersParamsParams {
    #[serde(rename = "isImage")]
    pub is_image: bool,
    #[serde(rename = "answer")]
    pub answer: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPollAddParams {
    #[serde(rename = "question")]
    pub question: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    #[serde(rename = "answers")]
    pub answers: Vec<BlogPollAddParamsAnswersParamsParams>,
}
impl Schema for BlogPollAddParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"question\":{\"type\":\"string\"},\"endDate\":{\"type\":\"string\"},\"answers\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"isImage\":{\"type\":\"boolean\"},\"answer\":{\"type\":\"string\"}},\"required\":[\"isImage\",\"answer\"]}}},\"required\":[\"question\",\"answers\",\"endDate\"]}") . unwrap ()
    }
}
impl Agent for BlogPollAddParams {
    fn topic() -> &'static str {
        "blog_poll_add"
    }
    fn method() -> &'static str {
        "poll_add"
    }
    fn agent() -> &'static str {
        "blog"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPollAddReturns(pub bool);
impl Schema for BlogPollAddReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for BlogPollAddReturns {
    fn topic() -> &'static str {
        "blog_poll_add"
    }
    fn method() -> &'static str {
        "poll_add"
    }
    fn agent() -> &'static str {
        "blog"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
