// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogVoteGetAllVotesByBlogIdParams {
    #[serde(rename = "id")]
    pub id: f64,
}
impl Schema for BlogVoteGetAllVotesByBlogIdParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}")
    }
}
impl Agent for BlogVoteGetAllVotesByBlogIdParams {
    fn topic() -> &'static str {
        "blog_vote_getAllVotesByBlogId"
    }
    fn method() -> &'static str {
        "vote_getAllVotesByBlogId"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogVoteGetAllVotesByBlogIdReturnsParams {
    #[serde(rename = "count")]
    pub count: String,
    #[serde(rename = "answer")]
    pub answer: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogVoteGetAllVotesByBlogIdReturns(pub Vec<BlogVoteGetAllVotesByBlogIdReturnsParams>);
impl Schema for BlogVoteGetAllVotesByBlogIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"count\":{\"type\":\"string\"},\"answer\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"answer\",\"count\"]}}")
    }
}
impl Agent for BlogVoteGetAllVotesByBlogIdReturns {
    fn topic() -> &'static str {
        "blog_vote_getAllVotesByBlogId"
    }
    fn method() -> &'static str {
        "vote_getAllVotesByBlogId"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
