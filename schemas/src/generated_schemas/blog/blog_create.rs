// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogCreateParamsPropsParams {
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "metaDescription")]
    pub meta_description: String,
    #[serde(rename = "lang")]
    pub lang: Option<String>,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "metaTitle")]
    pub meta_title: String,
    #[serde(rename = "title")]
    pub title: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogCreateParams {
    #[serde(rename = "props")]
    pub props: BlogBlogCreateParamsPropsParams,
    #[serde(rename = "categoryId")]
    pub category_id: f64,
}
impl Schema for BlogBlogCreateParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"props\":{\"type\":\"object\",\"properties\":{\"image\":{\"type\":\"string\"},\"url\":{\"type\":\"string\"},\"metaDescription\":{\"type\":\"string\"},\"lang\":{\"type\":\"string\"},\"shortDescription\":{\"type\":\"string\"},\"content\":{\"type\":\"string\"},\"metaTitle\":{\"type\":\"string\"},\"title\":{\"type\":\"string\"}},\"required\":[\"title\",\"metaTitle\",\"url\",\"image\",\"shortDescription\",\"metaDescription\",\"content\"]},\"categoryId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"props\",\"categoryId\"]}")
    }
}
impl Agent for BlogBlogCreateParams {
    fn topic() -> &'static str {
        "blog_blog_create"
    }
    fn method() -> &'static str {
        "blog_create"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogCreateReturns(pub bool);
impl Schema for BlogBlogCreateReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for BlogBlogCreateReturns {
    fn topic() -> &'static str {
        "blog_blog_create"
    }
    fn method() -> &'static str {
        "blog_create"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
