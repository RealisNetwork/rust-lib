// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceGetItemsByFilterParamsOrderParams {
    #[serde(rename = "desc")]
    pub desc: bool,
    #[serde(rename = "name")]
    pub name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceGetItemsByFilterParamsPriceParams {
    #[serde(rename = "max")]
    pub max: i32,
    #[serde(rename = "min")]
    pub min: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceGetItemsByFilterParams {
    #[serde(rename = "additionalParam")]
    pub additional_param: Option<String>,
    #[serde(rename = "page")]
    pub page: i32,
    #[serde(rename = "order")]
    pub order: Option<MarketPlaceMarketPlaceGetItemsByFilterParamsOrderParams>,
    #[serde(rename = "price")]
    pub price: Option<MarketPlaceMarketPlaceGetItemsByFilterParamsPriceParams>,
    #[serde(rename = "additionalValue")]
    pub additional_value: Option<String>,
    #[serde(rename = "category")]
    pub category: Option<i32>,
    #[serde(rename = "personalType")]
    pub personal_type: Option<String>,
    #[serde(rename = "perPage")]
    pub per_page: i32,
}
impl Schema for MarketPlaceMarketPlaceGetItemsByFilterParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"additionalParam\":{\"type\":\"string\"},\"page\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"order\":{\"type\":\"object\",\"properties\":{\"desc\":{\"type\":\"boolean\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"desc\"]},\"price\":{\"type\":\"object\",\"properties\":{\"max\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"min\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"min\",\"max\"]},\"additionalValue\":{\"type\":\"string\"},\"category\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"personalType\":{\"type\":\"string\"},\"perPage\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"perPage\",\"page\"]}")
    }
}
impl Agent for MarketPlaceMarketPlaceGetItemsByFilterParams {
    fn topic() -> &'static str {
        "market-place_marketPlace_getItemsByFilter"
    }
    fn method() -> &'static str {
        "marketPlace_getItemsByFilter"
    }
    fn agent() -> &'static str {
        "market-place"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceGetItemsByFilterReturnsParamsAdditionalParamsParams {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceGetItemsByFilterReturnsParams {
    #[serde(rename = "subType")]
    pub sub_type: String,
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "additionalParams")]
    pub additional_params:
        MarketPlaceMarketPlaceGetItemsByFilterReturnsParamsAdditionalParamsParams,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "productId")]
    pub product_id: i32,
    #[serde(rename = "category")]
    pub category: i32,
    #[serde(rename = "updatedAt")]
    pub updated_at: i32,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "price")]
    pub price: i32,
    #[serde(rename = "userId")]
    pub user_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceGetItemsByFilterReturns(
    pub Vec<MarketPlaceMarketPlaceGetItemsByFilterReturnsParams>,
);
impl Schema for MarketPlaceMarketPlaceGetItemsByFilterReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"subType\":{\"type\":\"string\"},\"personalType\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"additionalParams\":{\"type\":\"object\",\"properties\":{},\"required\":null},\"type\":{\"type\":\"string\"},\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"category\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"updatedAt\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"isLocked\":{\"type\":\"boolean\"},\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"price\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"id\",\"productId\",\"userId\",\"personalType\",\"type\",\"subType\",\"price\",\"additionalParams\",\"isLocked\",\"category\",\"createdAt\",\"updatedAt\"]}}")
    }
}
impl Agent for MarketPlaceMarketPlaceGetItemsByFilterReturns {
    fn topic() -> &'static str {
        "market-place_marketPlace_getItemsByFilter"
    }
    fn method() -> &'static str {
        "marketPlace_getItemsByFilter"
    }
    fn agent() -> &'static str {
        "market-place"
    }
}
