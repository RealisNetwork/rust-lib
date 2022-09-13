// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusConfigGetMembershipInfoParams {
    #[serde(rename = "id")]
    pub id: f64,
}
impl Schema for StatusConfigGetMembershipInfoParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}")
    }
}
impl Agent for StatusConfigGetMembershipInfoParams {
    fn topic() -> &'static str {
        "status_config_getMembershipInfo"
    }
    fn method() -> &'static str {
        "config_getMembershipInfo"
    }
    fn agent() -> &'static str {
        "status"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusConfigGetMembershipInfoReturns {
    #[serde(rename = "priceInLis")]
    pub price_in_lis: String,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "multiplier")]
    pub multiplier: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "membership")]
    pub membership: String,
    #[serde(rename = "maxCount")]
    pub max_count: String,
    #[serde(rename = "price")]
    pub price: String,
}
impl Schema for StatusConfigGetMembershipInfoReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"priceInLis\":{\"type\":\"string\"},\"isAvailable\":{\"type\":\"boolean\"},\"multiplier\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"membership\":{\"type\":\"string\"},\"maxCount\":{\"type\":\"string\"},\"price\":{\"type\":\"string\"}},\"required\":[\"id\",\"membership\",\"price\",\"multiplier\",\"priceInLis\",\"maxCount\",\"isAvailable\"]}")
    }
}
impl Agent for StatusConfigGetMembershipInfoReturns {
    fn topic() -> &'static str {
        "status_config_getMembershipInfo"
    }
    fn method() -> &'static str {
        "config_getMembershipInfo"
    }
    fn agent() -> &'static str {
        "status"
    }
}
