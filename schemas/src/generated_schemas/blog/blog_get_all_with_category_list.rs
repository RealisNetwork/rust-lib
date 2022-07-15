// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct BlogBlogGetAllWithCategoryListParams {
    #[serde(rename = "limit")]
    pub limit: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlogBlogGetAllWithCategoryListReturnsParamsArticlesParamsParams {
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlogBlogGetAllWithCategoryListReturnsParams {
    #[serde(rename = "articles")]
    pub articles: Vec<BlogBlogGetAllWithCategoryListReturnsParamsArticlesParamsParams>,
    #[serde(rename = "categoryId")]
    pub category_id: i64,
    #[serde(rename = "categoryName")]
    pub category_name: String,
}
pub type BlogBlogGetAllWithCategoryListReturns = Vec<BlogBlogGetAllWithCategoryListReturnsParams>;
