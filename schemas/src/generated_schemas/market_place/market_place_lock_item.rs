// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceLockItemParams {
    #[serde(rename = "recipient")]
    pub recipient: String,
    #[serde(rename = "productId")]
    pub product_id: i32,
}
impl Schema for MarketPlaceMarketPlaceLockItemParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"recipient\":{\"type\":\"string\"},\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"productId\",\"recipient\"]}") . unwrap ()
    }
}
impl Agent for MarketPlaceMarketPlaceLockItemParams {
    fn topic() -> &'static str {
        "market-place_marketPlace_lockItem"
    }
    fn method() -> &'static str {
        "marketPlace_lockItem"
    }
    fn agent() -> &'static str {
        "market-place"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceLockItemReturns(pub bool);
impl Schema for MarketPlaceMarketPlaceLockItemReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for MarketPlaceMarketPlaceLockItemReturns {
    fn topic() -> &'static str {
        "market-place_marketPlace_lockItem"
    }
    fn method() -> &'static str {
        "marketPlace_lockItem"
    }
    fn agent() -> &'static str {
        "market-place"
    }
}
