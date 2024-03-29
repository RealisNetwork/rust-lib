// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogGameNewsDeleteParams {
    #[serde(rename = "id")]
    pub id: f64,
}
impl Schema for BlogGameNewsDeleteParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}") . unwrap ()
    }
}
impl Agent for BlogGameNewsDeleteParams {
    fn topic() -> &'static str {
        "blog_gameNews_delete"
    }
    fn method() -> &'static str {
        "gameNews_delete"
    }
    fn agent() -> &'static str {
        "blog"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogGameNewsDeleteReturns(pub bool);
impl Schema for BlogGameNewsDeleteReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for BlogGameNewsDeleteReturns {
    fn topic() -> &'static str {
        "blog_gameNews_delete"
    }
    fn method() -> &'static str {
        "gameNews_delete"
    }
    fn agent() -> &'static str {
        "blog"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
