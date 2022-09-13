// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for MarketPlaceMarketPlaceGetCanceledItemsParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(MarketPlaceMarketPlaceGetCanceledItemsParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct MarketPlaceMarketPlaceGetCanceledItemsParams;
impl Schema for MarketPlaceMarketPlaceGetCanceledItemsParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for MarketPlaceMarketPlaceGetCanceledItemsParams {
    fn topic() -> &'static str {
        "market-place_marketPlace_getCanceledItems"
    }
    fn method() -> &'static str {
        "marketPlace_getCanceledItems"
    }
    fn agent() -> &'static str {
        "market-place"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceGetCanceledItemsReturnsParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "productId")]
    pub product_id: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceGetCanceledItemsReturns(
    pub Vec<MarketPlaceMarketPlaceGetCanceledItemsReturnsParams>,
);
impl Schema for MarketPlaceMarketPlaceGetCanceledItemsReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"status\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"id\",\"userId\",\"productId\",\"status\",\"createdAt\"]}}")
    }
}
impl Agent for MarketPlaceMarketPlaceGetCanceledItemsReturns {
    fn topic() -> &'static str {
        "market-place_marketPlace_getCanceledItems"
    }
    fn method() -> &'static str {
        "marketPlace_getCanceledItems"
    }
    fn agent() -> &'static str {
        "market-place"
    }
}
