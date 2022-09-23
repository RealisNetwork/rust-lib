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
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogGetByUrlReturns {
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "question")]
    pub question: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "metaTitle")]
    pub meta_title: String,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "lang")]
    pub lang: String,
    #[serde(rename = "isOpenPoll")]
    pub is_open_poll: bool,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "category")]
    pub category: BlogBlogGetByUrlReturnsCategoryParams,
    #[serde(rename = "isPinned")]
    pub is_pinned: bool,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "answers")]
    pub answers: Vec<String>,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "metaDescription")]
    pub meta_description: String,
    #[serde(rename = "views")]
    pub views: f64,
}
impl Schema for BlogBlogGetByUrlReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"image\":{\"type\":\"string\"},\"question\":{\"type\":\"string\"},\"title\":{\"type\":\"string\"},\"metaTitle\":{\"type\":\"string\"},\"shortDescription\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"lang\":{\"type\":\"string\"},\"isOpenPoll\":{\"type\":\"boolean\"},\"updatedAt\":{\"type\":\"string\"},\"category\":{\"type\":\"object\",\"properties\":{\"name\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"isAvailable\":{\"type\":\"boolean\"}},\"required\":[\"id\",\"name\",\"isAvailable\"]},\"isPinned\":{\"type\":\"boolean\"},\"url\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"content\":{\"type\":\"string\"},\"answers\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"isAvailable\":{\"type\":\"boolean\"},\"metaDescription\":{\"type\":\"string\"},\"views\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"title\",\"metaTitle\",\"url\",\"image\",\"shortDescription\",\"metaDescription\",\"content\",\"isPinned\",\"views\",\"isAvailable\",\"lang\",\"isOpenPoll\",\"question\",\"answers\",\"createdAt\",\"updatedAt\",\"category\"]}")
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
