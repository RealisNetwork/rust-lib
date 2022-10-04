// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPollUpdateParamsPropsParamsAnswersParamsParams {
    #[serde(rename = "isImage")]
    pub is_image: bool,
    #[serde(rename = "answer")]
    pub answer: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPollUpdateParamsPropsParams {
    #[serde(rename = "question")]
    pub question: Option<String>,
    #[serde(rename = "answers")]
    pub answers: Option<Vec<BlogPollUpdateParamsPropsParamsAnswersParamsParams>>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    #[serde(rename = "id")]
    pub id: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPollUpdateParams {
    #[serde(rename = "props")]
    pub props: BlogPollUpdateParamsPropsParams,
}
impl Schema for BlogPollUpdateParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"props\":{\"type\":\"object\",\"properties\":{\"question\":{\"type\":\"string\"},\"answers\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"isImage\":{\"type\":\"boolean\"},\"answer\":{\"type\":\"string\"}},\"required\":[\"isImage\",\"answer\"]}},\"endDate\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}},\"required\":[\"props\"]}") . unwrap ()
    }
}
impl Agent for BlogPollUpdateParams {
    fn topic() -> &'static str {
        "blog_poll_update"
    }
    fn method() -> &'static str {
        "poll_update"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPollUpdateReturns(pub bool);
impl Schema for BlogPollUpdateReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for BlogPollUpdateReturns {
    fn topic() -> &'static str {
        "blog_poll_update"
    }
    fn method() -> &'static str {
        "poll_update"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
