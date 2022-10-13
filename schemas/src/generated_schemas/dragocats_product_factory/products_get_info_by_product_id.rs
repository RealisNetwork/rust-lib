// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsProductFactoryProductsGetInfoByProductIdParams {
    #[serde(rename = "productId")]
    pub product_id: f64,
}
impl Schema for DragocatsProductFactoryProductsGetInfoByProductIdParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"productId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"productId\"]}") . unwrap ()
    }
}
impl Agent for DragocatsProductFactoryProductsGetInfoByProductIdParams {
    fn topic() -> &'static str {
        "dragocats-product-factory_products_getInfoByProductId"
    }
    fn method() -> &'static str {
        "products_getInfoByProductId"
    }
    fn agent() -> &'static str {
        "dragocats-product-factory"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsProductFactoryProductsGetInfoByProductIdReturns {
    #[serde(rename = "isNft")]
    pub is_nft: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "subType")]
    pub sub_type: String,
    #[serde(rename = "productType")]
    pub product_type: String,
}
impl Schema for DragocatsProductFactoryProductsGetInfoByProductIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"isNft\":{\"type\":\"boolean\"},\"name\":{\"type\":\"string\"},\"personalType\":{\"type\":\"string\"},\"subType\":{\"type\":\"string\"},\"productType\":{\"type\":\"string\"}},\"required\":[\"personalType\",\"productType\",\"subType\",\"name\",\"isNft\"]}")
    }
}
impl Agent for DragocatsProductFactoryProductsGetInfoByProductIdReturns {
    fn topic() -> &'static str {
        "dragocats-product-factory_products_getInfoByProductId"
    }
    fn method() -> &'static str {
        "products_getInfoByProductId"
    }
    fn agent() -> &'static str {
        "dragocats-product-factory"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
