// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for BlogCategoryGetAllParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(BlogCategoryGetAllParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct BlogCategoryGetAllParams;
impl Schema for BlogCategoryGetAllParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for BlogCategoryGetAllParams {
    fn topic() -> &'static str {
        "blog_category_getAll"
    }
    fn method() -> &'static str {
        "category_getAll"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogCategoryGetAllReturnsParams {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "name")]
    pub name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogCategoryGetAllReturns(pub Vec<BlogCategoryGetAllReturnsParams>);
impl Schema for BlogCategoryGetAllReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"isAvailable\":{\"type\":\"boolean\"},\"name\":{\"type\":\"string\"}},\"required\":[\"id\",\"name\",\"isAvailable\"]}}")
    }
}
impl Agent for BlogCategoryGetAllReturns {
    fn topic() -> &'static str {
        "blog_category_getAll"
    }
    fn method() -> &'static str {
        "category_getAll"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
