// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceAddToMarketPlaceParamsAdditionalParamsParams {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceAddToMarketPlaceParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "price")]
    pub price: i32,
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "productId")]
    pub product_id: i32,
    #[serde(rename = "additionalParams")]
    pub additional_params: MarketPlaceMarketPlaceAddToMarketPlaceParamsAdditionalParamsParams,
    #[serde(rename = "category")]
    pub category: String,
}
impl Schema for MarketPlaceMarketPlaceAddToMarketPlaceParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"price\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"personalType\":{\"type\":\"string\"},\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"additionalParams\":{\"type\":\"object\",\"properties\":{}},\"category\":{\"type\":\"string\"}},\"required\":[\"userId\",\"price\",\"category\",\"personalType\",\"productId\",\"additionalParams\"]}") . unwrap ()
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
