// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogCategoryUpdateParams {
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "isAvailable")]
    pub is_available: Option<bool>,
}
impl Schema for BlogCategoryUpdateParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"name\":{\"type\":\"string\"},\"isAvailable\":{\"type\":\"boolean\"}},\"required\":[\"id\",\"name\"]}")
    }
}
impl Agent for BlogCategoryUpdateParams {
    fn topic() -> &'static str {
        "blog_category_update"
    }
    fn method() -> &'static str {
        "category_update"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogCategoryUpdateReturns(pub bool);
impl Schema for BlogCategoryUpdateReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for BlogCategoryUpdateReturns {
    fn topic() -> &'static str {
        "blog_category_update"
    }
    fn method() -> &'static str {
        "category_update"
    }
    fn agent() -> &'static str {
        "blog"
    }
}
