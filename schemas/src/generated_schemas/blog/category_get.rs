// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogCategoryGetParams {
    #[serde(rename = "id")]
    pub id: f64,
}
impl Schema for BlogCategoryGetParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}") . unwrap ()
    }
}
impl Agent for BlogCategoryGetParams {
    fn topic() -> &'static str {
        "blog_category_get"
    }
    fn method() -> &'static str {
        "category_get"
    }
    fn agent() -> &'static str {
        "blog"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogCategoryGetReturns {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "id")]
    pub id: f64,
}
impl Schema for BlogCategoryGetReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"name\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"name\"]}")
    }
}
impl Agent for BlogCategoryGetReturns {
    fn topic() -> &'static str {
        "blog_category_get"
    }
    fn method() -> &'static str {
        "category_get"
    }
    fn agent() -> &'static str {
        "blog"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
