// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryProductsGetInfoByProductIdParams {
    #[serde(rename = "productId")]
    pub product_id: f64,
}
impl Schema for ProductFactoryProductsGetInfoByProductIdParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"productId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"productId\"]}") . unwrap ()
    }
}
impl Agent for ProductFactoryProductsGetInfoByProductIdParams {
    fn topic() -> &'static str {
        "product-factory_products_getInfoByProductId"
    }
    fn method() -> &'static str {
        "products_getInfoByProductId"
    }
    fn agent() -> &'static str {
        "product-factory"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryProductsGetInfoByProductIdReturns {
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "productType")]
    pub product_type: String,
    #[serde(rename = "isNft")]
    pub is_nft: bool,
    #[serde(rename = "subType")]
    pub sub_type: String,
    #[serde(rename = "name")]
    pub name: String,
}
impl Schema for ProductFactoryProductsGetInfoByProductIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"personalType\":{\"type\":\"string\"},\"productType\":{\"type\":\"string\"},\"isNft\":{\"type\":\"boolean\"},\"subType\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"personalType\",\"productType\",\"subType\",\"name\",\"isNft\"]}")
    }
}
impl Agent for ProductFactoryProductsGetInfoByProductIdReturns {
    fn topic() -> &'static str {
        "product-factory_products_getInfoByProductId"
    }
    fn method() -> &'static str {
        "products_getInfoByProductId"
    }
    fn agent() -> &'static str {
        "product-factory"
    }
}
