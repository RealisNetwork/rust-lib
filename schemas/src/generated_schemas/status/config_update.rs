// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusConfigUpdateParams {
    #[serde(rename = "priceInLis")]
    pub price_in_lis: String,
    #[serde(rename = "maxCount")]
    pub max_count: String,
    #[serde(rename = "multiplier")]
    pub multiplier: String,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "membership")]
    pub membership: String,
}
impl Schema for StatusConfigUpdateParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"priceInLis\":{\"type\":\"string\"},\"maxCount\":{\"type\":\"string\"},\"multiplier\":{\"type\":\"string\"},\"isAvailable\":{\"type\":\"boolean\"},\"price\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"membership\":{\"type\":\"string\"}},\"required\":[\"id\",\"membership\",\"price\",\"multiplier\",\"priceInLis\",\"maxCount\",\"isAvailable\"]}")
    }
}
impl Agent for StatusConfigUpdateParams {
    fn topic() -> &'static str {
        "status_config_update"
    }
    fn method() -> &'static str {
        "config_update"
    }
    fn agent() -> &'static str {
        "status"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusConfigUpdateReturns(bool);
impl Schema for StatusConfigUpdateReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for StatusConfigUpdateReturns {
    fn topic() -> &'static str {
        "status_config_update"
    }
    fn method() -> &'static str {
        "config_update"
    }
    fn agent() -> &'static str {
        "status"
    }
}
