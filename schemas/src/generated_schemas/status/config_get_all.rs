// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for StatusConfigGetAllParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(StatusConfigGetAllParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct StatusConfigGetAllParams;
impl Schema for StatusConfigGetAllParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusConfigGetAllReturnsParams {
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "membership")]
    pub membership: String,
    #[serde(rename = "membershipId")]
    pub membership_id: i64,
    #[serde(rename = "priceInLis")]
    pub price_in_lis: String,
    #[serde(rename = "maxCount")]
    pub max_count: String,
    #[serde(rename = "priorityIndex")]
    pub priority_index: i64,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "multiplier")]
    pub multiplier: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusConfigGetAllReturns(Vec<StatusConfigGetAllReturnsParams>);
impl Schema for StatusConfigGetAllReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"price\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"membership\":{\"type\":\"string\"},\"membershipId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"priceInLis\":{\"type\":\"string\"},\"maxCount\":{\"type\":\"string\"},\"priorityIndex\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"isAvailable\":{\"type\":\"boolean\"},\"multiplier\":{\"type\":\"string\"}},\"required\":[\"id\",\"membershipId\",\"priorityIndex\",\"membership\",\"price\",\"multiplier\",\"priceInLis\",\"maxCount\",\"isAvailable\"]}}")
    }
}