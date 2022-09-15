// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogGetAllWithCategoryListParams {
    #[serde(rename = "limit")]
    pub limit: Option<f64>,
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
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    #[serde(rename = "image")]
    pub image: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogGetAllWithCategoryListReturnsParams {
    #[serde(rename = "categoryName")]
    pub category_name: String,
    #[serde(rename = "categoryId")]
    pub category_id: f64,
    #[serde(rename = "articles")]
    pub articles: Vec<BlogBlogGetAllWithCategoryListReturnsParamsArticlesParamsParams>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogGetAllWithCategoryListReturns(
    pub Vec<BlogBlogGetAllWithCategoryListReturnsParams>,
);
impl Schema for BlogBlogGetAllWithCategoryListReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"categoryName\":{\"type\":\"string\"},\"categoryId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"articles\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"title\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"url\":{\"type\":\"string\"},\"shortDescription\":{\"type\":\"string\"},\"image\":{\"type\":\"string\"}},\"required\":[\"title\",\"image\",\"url\",\"createdAt\",\"shortDescription\"]}}},\"required\":[\"categoryName\",\"categoryId\",\"articles\"]}}")
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
