// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceAddToMarketPlaceParamsAdditionalParamsParams(Value);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceAddToMarketPlaceParams {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "additionalParams")]
    pub additional_params: MarketPlaceMarketPlaceAddToMarketPlaceParamsAdditionalParamsParams,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "price")]
    pub price: i32,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "productId")]
    pub product_id: i32,
    #[serde(rename = "collection")]
    pub collection: String,
}
impl Schema for MarketPlaceMarketPlaceAddToMarketPlaceParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"category\":{\"type\":\"string\"},\"additionalParams\":{\"type\":\"object\",\"properties\":{}},\"image\":{\"type\":\"string\"},\"price\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"},\"personalType\":{\"type\":\"string\"},\"description\":{\"type\":\"string\"},\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"collection\":{\"type\":\"string\"}},\"required\":[\"collection\",\"userId\",\"price\",\"description\",\"image\",\"category\",\"personalType\",\"productId\",\"additionalParams\"]}") . unwrap ()
    }
}
impl Agent for MarketPlaceMarketPlaceAddToMarketPlaceParams {
    fn topic() -> &'static str {
        "market-place_marketPlace_addToMarketPlace"
    }
    fn method() -> &'static str {
        "marketPlace_addToMarketPlace"
    }
    fn agent() -> &'static str {
        "market-place"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceAddToMarketPlaceReturns(pub bool);
impl Schema for MarketPlaceMarketPlaceAddToMarketPlaceReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for MarketPlaceMarketPlaceAddToMarketPlaceReturns {
    fn topic() -> &'static str {
        "market-place_marketPlace_addToMarketPlace"
    }
    fn method() -> &'static str {
        "marketPlace_addToMarketPlace"
    }
    fn agent() -> &'static str {
        "market-place"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
