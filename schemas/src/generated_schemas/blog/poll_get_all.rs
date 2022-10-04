// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPollGetAllParams {
    #[serde(rename = "page")]
    pub page: f64,
    #[serde(rename = "perPage")]
    pub per_page: f64,
}
impl Schema for BlogPollGetAllParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"page\",\"perPage\"]}") . unwrap ()
    }
}
impl Agent for BlogPollGetAllParams {
    fn topic() -> &'static str {
        "blog_poll_getAll"
    }
    fn method() -> &'static str {
        "poll_getAll"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPollGetAllReturnsDataParamsParamsAnswersParamsParams {
    #[serde(rename = "answer")]
    pub answer: String,
    #[serde(rename = "isImage")]
    pub is_image: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPollGetAllReturnsDataParamsParams {
    #[serde(rename = "question")]
    pub question: String,
    #[serde(rename = "answers")]
    pub answers: Vec<BlogPollGetAllReturnsDataParamsParamsAnswersParamsParams>,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "endDate")]
    pub end_date: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPollGetAllReturns {
    #[serde(rename = "data")]
    pub data: Vec<BlogPollGetAllReturnsDataParamsParams>,
    #[serde(rename = "totalCount")]
    pub total_count: f64,
}
impl Schema for BlogPollGetAllReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"data\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"question\":{\"type\":\"string\"},\"answers\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"answer\":{\"type\":\"string\"},\"isImage\":{\"type\":\"boolean\"}},\"required\":[\"isImage\",\"answer\"]}},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"endDate\":{\"type\":\"string\"}},\"required\":[\"id\",\"question\",\"answers\",\"endDate\"]}},\"totalCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"totalCount\",\"data\"]}")
    }
}
impl Agent for BlogPollGetAllReturns {
    fn topic() -> &'static str {
        "blog_poll_getAll"
    }
    fn method() -> &'static str {
        "poll_getAll"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
