// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceGetItemsByFilterParamsPriceParams {
    #[serde(rename = "min")]
    pub min: i32,
    #[serde(rename = "max")]
    pub max: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceGetItemsByFilterParamsOrderParams {
    #[serde(rename = "desc")]
    pub desc: bool,
    #[serde(rename = "name")]
    pub name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceGetItemsByFilterParams {
    #[serde(rename = "price")]
    pub price: Option<MarketPlaceMarketPlaceGetItemsByFilterParamsPriceParams>,
    #[serde(rename = "page")]
    pub page: i32,
    #[serde(rename = "additionalParam")]
    pub additional_param: Option<String>,
    #[serde(rename = "personalType")]
    pub personal_type: Option<String>,
    #[serde(rename = "perPage")]
    pub per_page: i32,
    #[serde(rename = "category")]
    pub category: Option<i32>,
    #[serde(rename = "additionalValue")]
    pub additional_value: Option<String>,
    #[serde(rename = "order")]
    pub order: Option<MarketPlaceMarketPlaceGetItemsByFilterParamsOrderParams>,
}
impl Schema for MarketPlaceMarketPlaceGetItemsByFilterParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"price\":{\"type\":\"object\",\"properties\":{\"min\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"max\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"min\",\"max\"]},\"page\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"additionalParam\":{\"type\":\"string\"},\"personalType\":{\"type\":\"string\"},\"perPage\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"category\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"additionalValue\":{\"type\":\"string\"},\"order\":{\"type\":\"object\",\"properties\":{\"desc\":{\"type\":\"boolean\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"desc\"]}},\"required\":[\"perPage\",\"page\"]}") . unwrap ()
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
    #[serde(rename = "updatedAt")]
    pub updated_at: i32,
    #[serde(rename = "price")]
    pub price: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "additionalParams")]
    pub additional_params:
        MarketPlaceMarketPlaceGetItemsByFilterReturnsParamsAdditionalParamsParams,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "productId")]
    pub product_id: i32,
    #[serde(rename = "subType")]
    pub sub_type: String,
    #[serde(rename = "category")]
    pub category: i32,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceGetItemsByFilterReturns(
    pub Vec<MarketPlaceMarketPlaceGetItemsByFilterReturnsParams>,
);
impl Schema for MarketPlaceMarketPlaceGetItemsByFilterReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"updatedAt\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"price\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"},\"type\":{\"type\":\"string\"},\"additionalParams\":{\"type\":\"object\",\"properties\":{}},\"isLocked\":{\"type\":\"boolean\"},\"personalType\":{\"type\":\"string\"},\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"subType\":{\"type\":\"string\"},\"category\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"createdAt\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"id\",\"productId\",\"userId\",\"personalType\",\"type\",\"subType\",\"price\",\"additionalParams\",\"isLocked\",\"category\",\"createdAt\",\"updatedAt\"]}}")
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
