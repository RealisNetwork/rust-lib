// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for StatusConfigGetAllParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(StatusConfigGetAllParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct StatusConfigGetAllParams;
impl Schema for StatusConfigGetAllParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for StatusConfigGetAllParams {
    fn topic() -> &'static str {
        "status_config_getAll"
    }
    fn method() -> &'static str {
        "config_getAll"
    }
    fn agent() -> &'static str {
        "status"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusConfigGetAllReturnsParams {
    #[serde(rename = "priorityIndex")]
    pub priority_index: f64,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "membershipId")]
    pub membership_id: f64,
    #[serde(rename = "multiplier")]
    pub multiplier: String,
    #[serde(rename = "priceInLis")]
    pub price_in_lis: String,
    #[serde(rename = "membership")]
    pub membership: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "maxCount")]
    pub max_count: String,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusConfigGetAllReturns(pub Vec<StatusConfigGetAllReturnsParams>);
impl Schema for StatusConfigGetAllReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"priorityIndex\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"membershipId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"multiplier\":{\"type\":\"string\"},\"priceInLis\":{\"type\":\"string\"},\"membership\":{\"type\":\"string\"},\"price\":{\"type\":\"string\"},\"maxCount\":{\"type\":\"string\"},\"isAvailable\":{\"type\":\"boolean\"}},\"required\":[\"id\",\"membershipId\",\"priorityIndex\",\"membership\",\"price\",\"multiplier\",\"priceInLis\",\"maxCount\",\"isAvailable\"]}}")
    }
}
impl Agent for StatusConfigGetAllReturns {
    fn topic() -> &'static str {
        "status_config_getAll"
    }
    fn method() -> &'static str {
        "config_getAll"
    }
    fn agent() -> &'static str {
        "status"
    }
}
