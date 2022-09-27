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
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"productId\"]}")
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
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "price")]
    pub price: i32,
    #[serde(rename = "subType")]
    pub sub_type: String,
    #[serde(rename = "additionalParams")]
    pub additional_params: MarketPlaceMarketEndpointsGetSimilarReturnsParamsAdditionalParamsParams,
    #[serde(rename = "category")]
    pub category: i32,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "productId")]
    pub product_id: i32,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketEndpointsGetSimilarReturns(
    pub Vec<MarketPlaceMarketEndpointsGetSimilarReturnsParams>,
);
impl Schema for MarketPlaceMarketEndpointsGetSimilarReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"type\":{\"type\":\"string\"},\"price\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"subType\":{\"type\":\"string\"},\"additionalParams\":{\"type\":\"object\",\"properties\":{},\"required\":null},\"category\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"personalType\":{\"type\":\"string\"},\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"isLocked\":{\"type\":\"boolean\"}},\"required\":[\"id\",\"productId\",\"userId\",\"personalType\",\"type\",\"subType\",\"price\",\"additionalParams\",\"isLocked\",\"category\",\"createdAt\"]}}")
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
