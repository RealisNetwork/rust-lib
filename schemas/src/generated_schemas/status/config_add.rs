// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusConfigAddParams {
    #[serde(rename = "maxCount", deserialize_with = "deserialize_to_string")]
    pub max_count: String,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "membership", deserialize_with = "deserialize_to_string")]
    pub membership: String,
    #[serde(rename = "priceInLis", deserialize_with = "deserialize_to_string")]
    pub price_in_lis: String,
    #[serde(rename = "price", deserialize_with = "deserialize_to_string")]
    pub price: String,
    #[serde(rename = "multiplier", deserialize_with = "deserialize_to_string")]
    pub multiplier: String,
}
impl Schema for StatusConfigAddParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"maxCount\":{\"type\":\"string\"},\"isAvailable\":{\"type\":\"boolean\"},\"membership\":{\"type\":\"string\"},\"priceInLis\":{\"type\":\"string\"},\"price\":{\"type\":\"string\"},\"multiplier\":{\"type\":\"string\"}},\"required\":[\"membership\",\"price\",\"multiplier\",\"priceInLis\",\"maxCount\",\"isAvailable\"]}") . unwrap ()
    }
}
impl Agent for StatusConfigAddParams {
    fn topic() -> &'static str {
        "status_config_add"
    }
    fn method() -> &'static str {
        "config_add"
    }
    fn agent() -> &'static str {
        "status"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusConfigAddReturns(pub bool);
impl Schema for StatusConfigAddReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for StatusConfigAddReturns {
    fn topic() -> &'static str {
        "status_config_add"
    }
    fn method() -> &'static str {
        "config_add"
    }
    fn agent() -> &'static str {
        "status"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
