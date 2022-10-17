// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPollGetParams {
    #[serde(rename = "id")]
    pub id: f64,
}
impl Schema for BlogPollGetParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}") . unwrap ()
    }
}
impl Agent for BlogPollGetParams {
    fn topic() -> &'static str {
        "blog_poll_get"
    }
    fn method() -> &'static str {
        "poll_get"
    }
    fn agent() -> &'static str {
        "blog"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPollGetReturnsAnswersParamsParams {
    #[serde(rename = "isImage")]
    pub is_image: bool,
    #[serde(rename = "answer")]
    pub answer: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPollGetReturns {
    #[serde(rename = "question")]
    pub question: String,
    #[serde(rename = "answers")]
    pub answers: Vec<BlogPollGetReturnsAnswersParamsParams>,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "endDate")]
    pub end_date: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}
impl Schema for BlogPollGetReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"question\":{\"type\":\"string\"},\"answers\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"isImage\":{\"type\":\"boolean\"},\"answer\":{\"type\":\"string\"}},\"required\":[\"isImage\",\"answer\"]}},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"endDate\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"}},\"required\":[\"id\",\"question\",\"answers\",\"endDate\",\"createdAt\",\"updatedAt\"]}")
    }
}
impl Agent for BlogPollGetReturns {
    fn topic() -> &'static str {
        "blog_poll_get"
    }
    fn method() -> &'static str {
        "poll_get"
    }
    fn agent() -> &'static str {
        "blog"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
