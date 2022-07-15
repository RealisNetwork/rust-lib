// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct BlogBlogGetAllParams {
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "page")]
    pub page: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlogBlogGetAllReturnsDataParamsParams {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "views")]
    pub views: i64,
    #[serde(rename = "isPinned")]
    pub is_pinned: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlogBlogGetAllReturns {
    #[serde(rename = "data")]
    pub data: Vec<BlogBlogGetAllReturnsDataParamsParams>,
    #[serde(rename = "totalCount")]
    pub total_count: i64,
}
