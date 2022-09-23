// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogUpdateIsOpenPollParams {
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "status")]
    pub status: bool,
}
impl Schema for BlogBlogUpdateIsOpenPollParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"status\":{\"type\":\"boolean\"}},\"required\":[\"id\",\"status\"]}")
    }
}
impl Agent for BlogBlogUpdateIsOpenPollParams {
    fn topic() -> &'static str {
        "blog_blog_updateIsOpenPoll"
    }
    fn method() -> &'static str {
        "blog_updateIsOpenPoll"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogUpdateIsOpenPollReturns(pub bool);
impl Schema for BlogBlogUpdateIsOpenPollReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for BlogBlogUpdateIsOpenPollReturns {
    fn topic() -> &'static str {
        "blog_blog_updateIsOpenPoll"
    }
    fn method() -> &'static str {
        "blog_updateIsOpenPoll"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
