// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogGameNewsCreateParams {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "appId")]
    pub app_id: f64,
    #[serde(rename = "image")]
    pub image: String,
}
impl Schema for BlogGameNewsCreateParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"title\":{\"type\":\"string\"},\"content\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"image\":{\"type\":\"string\"}},\"required\":[\"title\",\"content\",\"image\",\"appId\"]}") . unwrap ()
    }
}
impl Agent for BlogGameNewsCreateParams {
    fn topic() -> &'static str {
        "blog_gameNews_create"
    }
    fn method() -> &'static str {
        "gameNews_create"
    }
    fn agent() -> &'static str {
        "blog"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogGameNewsCreateReturns(pub bool);
impl Schema for BlogGameNewsCreateReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for BlogGameNewsCreateReturns {
    fn topic() -> &'static str {
        "blog_gameNews_create"
    }
    fn method() -> &'static str {
        "gameNews_create"
    }
    fn agent() -> &'static str {
        "blog"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
