// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogGameNewsUpdateParams {
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "image")]
    pub image: Option<String>,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "content")]
    pub content: Option<String>,
    #[serde(rename = "appId")]
    pub app_id: Option<f64>,
}
impl Schema for BlogGameNewsUpdateParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"image\":{\"type\":\"string\"},\"title\":{\"type\":\"string\"},\"content\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}") . unwrap ()
    }
}
impl Agent for BlogGameNewsUpdateParams {
    fn topic() -> &'static str {
        "blog_gameNews_update"
    }
    fn method() -> &'static str {
        "gameNews_update"
    }
    fn agent() -> &'static str {
        "blog"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogGameNewsUpdateReturns(pub bool);
impl Schema for BlogGameNewsUpdateReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for BlogGameNewsUpdateReturns {
    fn topic() -> &'static str {
        "blog_gameNews_update"
    }
    fn method() -> &'static str {
        "gameNews_update"
    }
    fn agent() -> &'static str {
        "blog"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
