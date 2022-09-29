// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusConfigAddParams {
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "membership")]
    pub membership: String,
    #[serde(rename = "priceInLis")]
    pub price_in_lis: String,
    #[serde(rename = "multiplier")]
    pub multiplier: String,
    #[serde(rename = "maxCount")]
    pub max_count: String,
}
impl Schema for StatusConfigAddParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"isAvailable\":{\"type\":\"boolean\"},\"price\":{\"type\":\"string\"},\"membership\":{\"type\":\"string\"},\"priceInLis\":{\"type\":\"string\"},\"multiplier\":{\"type\":\"string\"},\"maxCount\":{\"type\":\"string\"}},\"required\":[\"membership\",\"price\",\"multiplier\",\"priceInLis\",\"maxCount\",\"isAvailable\"]}") . unwrap ()
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
}
