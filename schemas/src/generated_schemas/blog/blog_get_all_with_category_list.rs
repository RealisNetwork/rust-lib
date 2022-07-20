// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogGetAllWithCategoryListParams {
    #[serde(rename = "limit")]
    pub limit: i64,
}
impl Schema for BlogBlogGetAllWithCategoryListParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"limit\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":null}")
    }
}
impl Agent for BlogBlogGetAllWithCategoryListParams {
    fn topic() -> &'static str {
        "blog_blog_getAllWithCategoryList"
    }
    fn method() -> &'static str {
        "blog_getAllWithCategoryList"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogGetAllWithCategoryListReturnsParamsArticlesParamsParams {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogGetAllWithCategoryListReturnsParams {
    #[serde(rename = "categoryId")]
    pub category_id: i64,
    #[serde(rename = "articles")]
    pub articles: Vec<BlogBlogGetAllWithCategoryListReturnsParamsArticlesParamsParams>,
    #[serde(rename = "categoryName")]
    pub category_name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogGetAllWithCategoryListReturns(Vec<BlogBlogGetAllWithCategoryListReturnsParams>);
impl Schema for BlogBlogGetAllWithCategoryListReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"categoryId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"articles\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"createdAt\":{\"type\":\"string\"},\"image\":{\"type\":\"string\"},\"shortDescription\":{\"type\":\"string\"},\"title\":{\"type\":\"string\"},\"url\":{\"type\":\"string\"}},\"required\":[\"title\",\"image\",\"url\",\"createdAt\",\"shortDescription\"]}},\"categoryName\":{\"type\":\"string\"}},\"required\":[\"categoryName\",\"categoryId\",\"articles\"]}}")
    }
}
impl Agent for BlogBlogGetAllWithCategoryListReturns {
    fn topic() -> &'static str {
        "blog_blog_getAllWithCategoryList"
    }
    fn method() -> &'static str {
        "blog_getAllWithCategoryList"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
