// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogVoteDeleteAllVotesByBlogIdParams {
    #[serde(rename = "id")]
    pub id: f64,
}
impl Schema for BlogVoteDeleteAllVotesByBlogIdParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}") . unwrap ()
    }
}
impl Agent for BlogVoteDeleteAllVotesByBlogIdParams {
    fn topic() -> &'static str {
        "blog_vote_deleteAllVotesByBlogId"
    }
    fn method() -> &'static str {
        "vote_deleteAllVotesByBlogId"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogVoteDeleteAllVotesByBlogIdReturns(pub bool);
impl Schema for BlogVoteDeleteAllVotesByBlogIdReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for BlogVoteDeleteAllVotesByBlogIdReturns {
    fn topic() -> &'static str {
        "blog_vote_deleteAllVotesByBlogId"
    }
    fn method() -> &'static str {
        "vote_deleteAllVotesByBlogId"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
