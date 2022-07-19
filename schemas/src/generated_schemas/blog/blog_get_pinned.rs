// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for BlogBlogGetPinnedParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(BlogBlogGetPinnedParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct BlogBlogGetPinnedParams;
impl Schema for BlogBlogGetPinnedParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogBlogGetPinnedReturns {
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "url")]
    pub url: String,
}
impl Schema for BlogBlogGetPinnedReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"shortDescription\":{\"type\":\"string\"},\"title\":{\"type\":\"string\"},\"image\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"url\":{\"type\":\"string\"}},\"required\":[\"title\",\"image\",\"url\",\"createdAt\",\"shortDescription\"]}")
    }
}
