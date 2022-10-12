// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogCategoryUpdateParams {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "isAvailable")]
    pub is_available: Option<bool>,
    #[serde(rename = "id")]
    pub id: f64,
}
impl Schema for BlogCategoryUpdateParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"name\":{\"type\":\"string\"},\"isAvailable\":{\"type\":\"boolean\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"name\"]}") . unwrap ()
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
