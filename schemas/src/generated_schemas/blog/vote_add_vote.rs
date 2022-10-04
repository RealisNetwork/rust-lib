// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogVoteAddVoteParams {
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "answer")]
    pub answer: f64,
}
impl Schema for BlogVoteAddVoteParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"answer\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"answer\"]}") . unwrap ()
    }
}
impl Agent for BlogVoteAddVoteParams {
    fn topic() -> &'static str {
        "blog_vote_addVote"
    }
    fn method() -> &'static str {
        "vote_addVote"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogVoteAddVoteReturns(pub bool);
impl Schema for BlogVoteAddVoteReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for BlogVoteAddVoteReturns {
    fn topic() -> &'static str {
        "blog_vote_addVote"
    }
    fn method() -> &'static str {
        "vote_addVote"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
