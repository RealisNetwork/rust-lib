// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogGetByUrlParams {
    #[serde(rename = "url")]
    pub url: String,
}
impl Schema for BlogBlogGetByUrlParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"url\":{\"type\":\"string\"}},\"required\":[\"url\"]}")
    }
}
impl Agent for BlogBlogGetByUrlParams {
    fn topic() -> &'static str {
        "blog_blog_getByUrl"
    }
    fn method() -> &'static str {
        "blog_getByUrl"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogGetByUrlReturnsCategoryParams {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogGetByUrlReturns {
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "metaDescription")]
    pub meta_description: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "metaTitle")]
    pub meta_title: String,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "isPinned")]
    pub is_pinned: bool,
    #[serde(rename = "views")]
    pub views: i64,
    #[serde(rename = "category")]
    pub category: BlogBlogGetByUrlReturnsCategoryParams,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "lang")]
    pub lang: String,
}
impl Schema for BlogBlogGetByUrlReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"image\":{\"type\":\"string\"},\"metaDescription\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"url\":{\"type\":\"string\"},\"metaTitle\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"title\":{\"type\":\"string\"},\"isPinned\":{\"type\":\"boolean\"},\"views\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"category\":{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"name\":{\"type\":\"string\"},\"isAvailable\":{\"type\":\"boolean\"}},\"required\":[\"id\",\"name\",\"isAvailable\"]},\"content\":{\"type\":\"string\"},\"isAvailable\":{\"type\":\"boolean\"},\"shortDescription\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"lang\":{\"type\":\"string\"}},\"required\":[\"id\",\"title\",\"metaTitle\",\"url\",\"image\",\"shortDescription\",\"metaDescription\",\"content\",\"isPinned\",\"views\",\"isAvailable\",\"lang\",\"createdAt\",\"updatedAt\",\"category\"]}")
    }
}
impl Agent for BlogBlogGetByUrlReturns {
    fn topic() -> &'static str {
        "blog_blog_getByUrl"
    }
    fn method() -> &'static str {
        "blog_getByUrl"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
