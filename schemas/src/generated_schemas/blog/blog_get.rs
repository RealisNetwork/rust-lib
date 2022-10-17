// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogGetParams {
    #[serde(rename = "id")]
    pub id: f64,
}
impl Schema for BlogBlogGetParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}") . unwrap ()
    }
}
impl Agent for BlogBlogGetParams {
    fn topic() -> &'static str {
        "blog_blog_get"
    }
    fn method() -> &'static str {
        "blog_get"
    }
    fn agent() -> &'static str {
        "blog"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogGetReturnsCategoryParams {
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "name")]
    pub name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogGetReturns {
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "views")]
    pub views: f64,
    #[serde(rename = "isPinned")]
    pub is_pinned: bool,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "lang")]
    pub lang: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "metaTitle")]
    pub meta_title: String,
    #[serde(rename = "category")]
    pub category: BlogBlogGetReturnsCategoryParams,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "metaDescription")]
    pub meta_description: String,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
}
impl Schema for BlogBlogGetReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"content\":{\"type\":\"string\"},\"views\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"isPinned\":{\"type\":\"boolean\"},\"title\":{\"type\":\"string\"},\"isAvailable\":{\"type\":\"boolean\"},\"lang\":{\"type\":\"string\"},\"url\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"createdAt\":{\"type\":\"string\"},\"metaTitle\":{\"type\":\"string\"},\"category\":{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"isAvailable\":{\"type\":\"boolean\"},\"name\":{\"type\":\"string\"}},\"required\":[\"id\",\"name\",\"isAvailable\"]},\"image\":{\"type\":\"string\"},\"metaDescription\":{\"type\":\"string\"},\"shortDescription\":{\"type\":\"string\"}},\"required\":[\"id\",\"title\",\"metaTitle\",\"url\",\"image\",\"shortDescription\",\"metaDescription\",\"content\",\"isPinned\",\"views\",\"isAvailable\",\"lang\",\"createdAt\",\"updatedAt\",\"category\"]}")
    }
}
impl Agent for BlogBlogGetReturns {
    fn topic() -> &'static str {
        "blog_blog_get"
    }
    fn method() -> &'static str {
        "blog_get"
    }
    fn agent() -> &'static str {
        "blog"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
