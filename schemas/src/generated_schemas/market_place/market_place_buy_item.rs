// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceBuyItemParams {
    #[serde(rename = "productId")]
    pub product_id: i32,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for MarketPlaceMarketPlaceBuyItemParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"productId\"]}")
    }
}
impl Agent for MarketPlaceMarketPlaceBuyItemParams {
    fn topic() -> &'static str {
        "market-place_marketPlace_buyItem"
    }
    fn method() -> &'static str {
        "marketPlace_buyItem"
    }
    fn agent() -> &'static str {
        "market-place"
    }
}
impl<'de> Deserialize<'de> for MarketPlaceMarketPlaceBuyItemReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(MarketPlaceMarketPlaceBuyItemReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct MarketPlaceMarketPlaceBuyItemReturns;
impl Schema for MarketPlaceMarketPlaceBuyItemReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for MarketPlaceMarketPlaceBuyItemReturns {
    fn topic() -> &'static str {
        "market-place_marketPlace_buyItem"
    }
    fn method() -> &'static str {
        "marketPlace_buyItem"
    }
    fn agent() -> &'static str {
        "market-place"
    }
}
