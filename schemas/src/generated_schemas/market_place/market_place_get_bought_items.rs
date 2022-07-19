// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for MarketPlaceMarketPlaceGetBoughtItemsParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(MarketPlaceMarketPlaceGetBoughtItemsParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct MarketPlaceMarketPlaceGetBoughtItemsParams;
impl Schema for MarketPlaceMarketPlaceGetBoughtItemsParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for MarketPlaceMarketPlaceGetBoughtItemsParams {
    fn topic() -> &'static str {
        "market-place_marketPlace_getBoughtItems"
    }
    fn method() -> &'static str {
        "marketPlace_getBoughtItems"
    }
    fn agent() -> &'static str {
        "market-place"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceGetBoughtItemsReturnsParams {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "productId")]
    pub product_id: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceGetBoughtItemsReturns(
    Vec<MarketPlaceMarketPlaceGetBoughtItemsReturnsParams>,
);
impl Schema for MarketPlaceMarketPlaceGetBoughtItemsReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"status\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"id\",\"userId\",\"productId\",\"status\",\"createdAt\"]}}")
    }
}
impl Agent for MarketPlaceMarketPlaceGetBoughtItemsReturns {
    fn topic() -> &'static str {
        "market-place_marketPlace_getBoughtItems"
    }
    fn method() -> &'static str {
        "marketPlace_getBoughtItems"
    }
    fn agent() -> &'static str {
        "market-place"
    }
}
