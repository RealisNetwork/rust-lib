// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for BlogBlogGetPinnedParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(BlogBlogGetPinnedParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct BlogBlogGetPinnedParams;
impl Schema for BlogBlogGetPinnedParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for BlogBlogGetPinnedParams {
    fn topic() -> &'static str {
        "blog_blog_getPinned"
    }
    fn method() -> &'static str {
        "blog_getPinned"
    }
    fn agent() -> &'static str {
        "blog"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogGetPinnedReturns {
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
}
impl Schema for BlogBlogGetPinnedReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"image\":{\"type\":\"string\"},\"url\":{\"type\":\"string\"},\"title\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"shortDescription\":{\"type\":\"string\"}},\"required\":[\"title\",\"image\",\"url\",\"createdAt\",\"shortDescription\"]}")
    }
}
impl Agent for BlogBlogGetPinnedReturns {
    fn topic() -> &'static str {
        "blog_blog_getPinned"
    }
    fn method() -> &'static str {
        "blog_getPinned"
    }
    fn agent() -> &'static str {
        "blog"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
