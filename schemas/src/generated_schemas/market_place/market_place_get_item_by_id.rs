// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceGetItemByIdParams {
    #[serde(rename = "productId")]
    pub product_id: i32,
}
impl Schema for MarketPlaceMarketPlaceGetItemByIdParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"productId\"]}") . unwrap ()
    }
}
impl Agent for MarketPlaceMarketPlaceGetItemByIdParams {
    fn topic() -> &'static str {
        "market-place_marketPlace_getItemById"
    }
    fn method() -> &'static str {
        "marketPlace_getItemById"
    }
    fn agent() -> &'static str {
        "market-place"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceGetItemByIdReturnsAdditionalParamsParams(Value);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceGetItemByIdReturns {
    #[serde(rename = "collection", deserialize_with = "deserialize_to_string")]
    pub collection: String,
    #[serde(rename = "description", deserialize_with = "deserialize_to_string")]
    pub description: String,
    #[serde(rename = "productId")]
    pub product_id: i32,
    #[serde(rename = "nickname", deserialize_with = "deserialize_to_string")]
    pub nickname: String,
    #[serde(rename = "personalType", deserialize_with = "deserialize_to_string")]
    pub personal_type: String,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    #[serde(rename = "additionalParams")]
    pub additional_params: MarketPlaceMarketPlaceGetItemByIdReturnsAdditionalParamsParams,
    #[serde(rename = "category")]
    pub category: i32,
    #[serde(rename = "image", deserialize_with = "deserialize_to_string")]
    pub image: String,
    #[serde(rename = "type", deserialize_with = "deserialize_to_string")]
    pub r#type: String,
    #[serde(rename = "price")]
    pub price: i32,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "updatedAt")]
    pub updated_at: i32,
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
    #[serde(rename = "id")]
    pub id: i32,
}
impl Schema for MarketPlaceMarketPlaceGetItemByIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"collection\":{\"type\":\"string\"},\"description\":{\"type\":\"string\"},\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"nickname\":{\"type\":\"string\"},\"personalType\":{\"type\":\"string\"},\"isLocked\":{\"type\":\"boolean\"},\"additionalParams\":{\"type\":\"object\",\"properties\":{}},\"category\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"image\":{\"type\":\"string\"},\"type\":{\"type\":\"string\"},\"price\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"createdAt\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"updatedAt\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"id\",\"productId\",\"userId\",\"nickname\",\"personalType\",\"type\",\"collection\",\"description\",\"image\",\"price\",\"additionalParams\",\"isLocked\",\"category\",\"createdAt\",\"updatedAt\"]}")
    }
}
impl Agent for MarketPlaceMarketPlaceGetItemByIdReturns {
    fn topic() -> &'static str {
        "market-place_marketPlace_getItemById"
    }
    fn method() -> &'static str {
        "marketPlace_getItemById"
    }
    fn agent() -> &'static str {
        "market-place"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
