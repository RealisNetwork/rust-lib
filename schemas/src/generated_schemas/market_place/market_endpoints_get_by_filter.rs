// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketEndpointsGetByFilterParamsFiltersParamsParams {
    #[serde(rename = "value")]
    pub value: (),
    #[serde(rename = "column")]
    pub column: String,
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
    #[serde(rename = "perPage")]
    pub per_page: i32,
    #[serde(rename = "page")]
    pub page: i32,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "filters")]
    pub filters: Option<Vec<MarketPlaceMarketEndpointsGetByFilterParamsFiltersParamsParams>>,
    #[serde(rename = "orderBy")]
    pub order_by: Option<MarketPlaceMarketEndpointsGetByFilterParamsOrderByParams>,
}
impl Schema for MarketPlaceMarketEndpointsGetByFilterParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"perPage\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"page\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"category\":{\"type\":\"string\"},\"filters\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"value\":{},\"column\":{\"type\":\"string\"}},\"required\":[\"column\",\"value\"]}},\"orderBy\":{\"type\":\"object\",\"properties\":{\"desc\":{\"type\":\"boolean\"},\"column\":{\"type\":\"string\"}},\"required\":[\"column\",\"desc\"]}},\"required\":[\"category\",\"page\",\"perPage\"]}")
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
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketEndpointsGetByFilterReturnsParamsAdditionalParamsParams {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketEndpointsGetByFilterReturnsParams {
    #[serde(rename = "productId")]
    pub product_id: i32,
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "subType")]
    pub sub_type: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "additionalParams")]
    pub additional_params: MarketPlaceMarketEndpointsGetByFilterReturnsParamsAdditionalParamsParams,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "category")]
    pub category: i32,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "price")]
    pub price: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketEndpointsGetByFilterReturns(
    pub Vec<MarketPlaceMarketEndpointsGetByFilterReturnsParams>,
);
impl Schema for MarketPlaceMarketEndpointsGetByFilterReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"personalType\":{\"type\":\"string\"},\"subType\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"additionalParams\":{\"type\":\"object\",\"properties\":{},\"required\":null},\"isLocked\":{\"type\":\"boolean\"},\"userId\":{\"type\":\"string\"},\"type\":{\"type\":\"string\"},\"category\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"createdAt\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"price\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"id\",\"productId\",\"userId\",\"personalType\",\"type\",\"subType\",\"price\",\"additionalParams\",\"isLocked\",\"category\",\"createdAt\"]}}")
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
}
