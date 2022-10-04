// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketEndpointsGetSimilarParams {
    #[serde(rename = "productId")]
    pub product_id: i32,
}
impl Schema for MarketPlaceMarketEndpointsGetSimilarParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"productId\"]}") . unwrap ()
    }
}
impl Agent for MarketPlaceMarketEndpointsGetSimilarParams {
    fn topic() -> &'static str {
        "market-place_marketEndpoints_getSimilar"
    }
    fn method() -> &'static str {
        "marketEndpoints_getSimilar"
    }
    fn agent() -> &'static str {
        "market-place"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketEndpointsGetSimilarReturnsParamsAdditionalParamsParams {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketEndpointsGetSimilarReturnsParams {
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "category")]
    pub category: i32,
    #[serde(rename = "additionalParams")]
    pub additional_params: MarketPlaceMarketEndpointsGetSimilarReturnsParamsAdditionalParamsParams,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    #[serde(rename = "subType")]
    pub sub_type: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "price")]
    pub price: i32,
    #[serde(rename = "productId")]
    pub product_id: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketEndpointsGetSimilarReturns(
    pub Vec<MarketPlaceMarketEndpointsGetSimilarReturnsParams>,
);
impl Schema for MarketPlaceMarketEndpointsGetSimilarReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"personalType\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"category\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"additionalParams\":{\"type\":\"object\",\"properties\":{}},\"isLocked\":{\"type\":\"boolean\"},\"subType\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"createdAt\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"type\":{\"type\":\"string\"},\"price\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"id\",\"productId\",\"userId\",\"personalType\",\"type\",\"subType\",\"price\",\"additionalParams\",\"isLocked\",\"category\",\"createdAt\"]}}")
    }
}
impl Agent for MarketPlaceMarketEndpointsGetSimilarReturns {
    fn topic() -> &'static str {
        "market-place_marketEndpoints_getSimilar"
    }
    fn method() -> &'static str {
        "marketEndpoints_getSimilar"
    }
    fn agent() -> &'static str {
        "market-place"
    }
}
