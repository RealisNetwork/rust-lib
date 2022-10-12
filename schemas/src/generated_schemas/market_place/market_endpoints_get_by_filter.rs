// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketEndpointsGetByFilterParamsFiltersParamsParams {
    #[serde(rename = "column")]
    pub column: String,
    #[serde(rename = "value")]
    pub value: (),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketEndpointsGetByFilterParamsOrderByParams {
    #[serde(rename = "desc")]
    pub desc: bool,
    #[serde(rename = "column")]
    pub column: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketEndpointsGetByFilterParams {
    #[serde(rename = "filters")]
    pub filters: Option<Vec<MarketPlaceMarketEndpointsGetByFilterParamsFiltersParamsParams>>,
    #[serde(rename = "perPage")]
    pub per_page: i32,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "orderBy")]
    pub order_by: Option<MarketPlaceMarketEndpointsGetByFilterParamsOrderByParams>,
    #[serde(rename = "page")]
    pub page: i32,
}
impl Schema for MarketPlaceMarketEndpointsGetByFilterParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"filters\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"column\":{\"type\":\"string\"},\"value\":{}},\"required\":[\"column\",\"value\"]}},\"perPage\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"category\":{\"type\":\"string\"},\"orderBy\":{\"type\":\"object\",\"properties\":{\"desc\":{\"type\":\"boolean\"},\"column\":{\"type\":\"string\"}},\"required\":[\"column\",\"desc\"]},\"page\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"category\",\"page\",\"perPage\"]}") . unwrap ()
    }
}
impl Agent for MarketPlaceMarketEndpointsGetByFilterParams {
    fn topic() -> &'static str {
        "market-place_marketEndpoints_getByFilter"
    }
    fn method() -> &'static str {
        "marketEndpoints_getByFilter"
    }
    fn agent() -> &'static str {
        "market-place"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketEndpointsGetByFilterReturnsParamsAdditionalParamsParams {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketEndpointsGetByFilterReturnsParams {
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    #[serde(rename = "category")]
    pub category: i32,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "subType")]
    pub sub_type: String,
    #[serde(rename = "price")]
    pub price: i32,
    #[serde(rename = "additionalParams")]
    pub additional_params: MarketPlaceMarketEndpointsGetByFilterReturnsParamsAdditionalParamsParams,
    #[serde(rename = "productId")]
    pub product_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "type")]
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketEndpointsGetByFilterReturns(
    pub Vec<MarketPlaceMarketEndpointsGetByFilterReturnsParams>,
);
impl Schema for MarketPlaceMarketEndpointsGetByFilterReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"isLocked\":{\"type\":\"boolean\"},\"category\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"createdAt\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"subType\":{\"type\":\"string\"},\"price\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"additionalParams\":{\"type\":\"object\",\"properties\":{}},\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"},\"personalType\":{\"type\":\"string\"},\"type\":{\"type\":\"string\"}},\"required\":[\"id\",\"productId\",\"userId\",\"personalType\",\"type\",\"subType\",\"price\",\"additionalParams\",\"isLocked\",\"category\",\"createdAt\"]}}")
    }
}
impl Agent for MarketPlaceMarketEndpointsGetByFilterReturns {
    fn topic() -> &'static str {
        "market-place_marketEndpoints_getByFilter"
    }
    fn method() -> &'static str {
        "marketEndpoints_getByFilter"
    }
    fn agent() -> &'static str {
        "market-place"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
