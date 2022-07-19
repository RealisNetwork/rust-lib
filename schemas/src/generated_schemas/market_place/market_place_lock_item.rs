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
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"recipient\":{\"type\":\"string\"},\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"productId\",\"recipient\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceLockItemReturns(bool);
impl Schema for MarketPlaceMarketPlaceLockItemReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}