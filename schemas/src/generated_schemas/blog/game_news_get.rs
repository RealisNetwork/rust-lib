// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogGameNewsGetParams {
    #[serde(rename = "id")]
    pub id: f64,
}
impl Schema for BlogGameNewsGetParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}") . unwrap ()
    }
}
impl Agent for BlogGameNewsGetParams {
    fn topic() -> &'static str {
        "blog_gameNews_get"
    }
    fn method() -> &'static str {
        "gameNews_get"
    }
    fn agent() -> &'static str {
        "blog"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogGameNewsGetReturns {
    #[serde(rename = "title", deserialize_with = "deserialize_to_string")]
    pub title: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "content", deserialize_with = "deserialize_to_string")]
    pub content: String,
    #[serde(rename = "image", deserialize_with = "deserialize_to_string")]
    pub image: String,
    #[serde(rename = "createdAt", deserialize_with = "deserialize_to_string")]
    pub created_at: String,
    #[serde(rename = "updatedAt", deserialize_with = "deserialize_to_string")]
    pub updated_at: String,
    #[serde(rename = "appId")]
    pub app_id: f64,
}
impl Schema for BlogGameNewsGetReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"title\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"content\":{\"type\":\"string\"},\"image\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"title\",\"content\",\"image\",\"appId\",\"createdAt\",\"updatedAt\"]}")
    }
}
impl Agent for BlogGameNewsGetReturns {
    fn topic() -> &'static str {
        "blog_gameNews_get"
    }
    fn method() -> &'static str {
        "gameNews_get"
    }
    fn agent() -> &'static str {
        "blog"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
