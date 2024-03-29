// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for StatusConfigGetAllForPurchaseParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(StatusConfigGetAllForPurchaseParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct StatusConfigGetAllForPurchaseParams;
impl Schema for StatusConfigGetAllForPurchaseParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for StatusConfigGetAllForPurchaseParams {
    fn topic() -> &'static str {
        "status_config_getAllForPurchase"
    }
    fn method() -> &'static str {
        "config_getAllForPurchase"
    }
    fn agent() -> &'static str {
        "status"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusConfigGetAllForPurchaseReturnsParams {
    #[serde(rename = "priorityIndex")]
    pub priority_index: f64,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "membership")]
    pub membership: String,
    #[serde(rename = "maxCount")]
    pub max_count: String,
    #[serde(rename = "priceInLis")]
    pub price_in_lis: String,
    #[serde(rename = "membershipId")]
    pub membership_id: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusConfigGetAllForPurchaseReturns(
    pub Vec<StatusConfigGetAllForPurchaseReturnsParams>,
);
impl Schema for StatusConfigGetAllForPurchaseReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"priorityIndex\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"price\":{\"type\":\"string\"},\"membership\":{\"type\":\"string\"},\"maxCount\":{\"type\":\"string\"},\"priceInLis\":{\"type\":\"string\"},\"membershipId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"membershipId\",\"priorityIndex\",\"membership\",\"maxCount\",\"priceInLis\",\"price\"]}}")
    }
}
impl Agent for StatusConfigGetAllForPurchaseReturns {
    fn topic() -> &'static str {
        "status_config_getAllForPurchase"
    }
    fn method() -> &'static str {
        "config_getAllForPurchase"
    }
    fn agent() -> &'static str {
        "status"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
