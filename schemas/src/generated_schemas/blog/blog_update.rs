// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogUpdateParamsPropsParams {
    #[serde(rename = "metaTitle")]
    pub meta_title: Option<String>,
    #[serde(rename = "metaDescription")]
    pub meta_description: Option<String>,
    #[serde(rename = "image")]
    pub image: Option<String>,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "lang")]
    pub lang: Option<String>,
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(rename = "shortDescription")]
    pub short_description: Option<String>,
    #[serde(rename = "content")]
    pub content: Option<String>,
    #[serde(rename = "isPinned")]
    pub is_pinned: Option<bool>,
    #[serde(rename = "id")]
    pub id: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogUpdateParams {
    #[serde(rename = "categoryId")]
    pub category_id: Option<f64>,
    #[serde(rename = "props")]
    pub props: BlogBlogUpdateParamsPropsParams,
}
impl Schema for BlogBlogUpdateParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"categoryId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"props\":{\"type\":\"object\",\"properties\":{\"metaTitle\":{\"type\":\"string\"},\"metaDescription\":{\"type\":\"string\"},\"image\":{\"type\":\"string\"},\"title\":{\"type\":\"string\"},\"lang\":{\"type\":\"string\"},\"url\":{\"type\":\"string\"},\"shortDescription\":{\"type\":\"string\"},\"content\":{\"type\":\"string\"},\"isPinned\":{\"type\":\"boolean\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}},\"required\":[\"props\"]}") . unwrap ()
    }
}
impl Agent for BlogBlogUpdateParams {
    fn topic() -> &'static str {
        "blog_blog_update"
    }
    fn method() -> &'static str {
        "blog_update"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogUpdateReturns(pub bool);
impl Schema for BlogBlogUpdateReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for BlogBlogUpdateReturns {
    fn topic() -> &'static str {
        "blog_blog_update"
    }
    fn method() -> &'static str {
        "blog_update"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
