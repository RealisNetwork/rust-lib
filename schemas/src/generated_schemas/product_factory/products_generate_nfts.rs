// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryProductsGenerateNftsParams {
    #[serde(rename = "collection", deserialize_with = "deserialize_to_string")]
    pub collection: String,
    #[serde(rename = "category", deserialize_with = "deserialize_to_string")]
    pub category: String,
}
impl Schema for ProductFactoryProductsGenerateNftsParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"collection\":{\"type\":\"string\"},\"category\":{\"type\":\"string\",\"pattern\":\"^(cats)|(dragons)$\"}},\"required\":[\"category\",\"collection\"]}") . unwrap ()
    }
}
impl Agent for ProductFactoryProductsGenerateNftsParams {
    fn topic() -> &'static str {
        "product-factory_products_generateNfts"
    }
    fn method() -> &'static str {
        "products_generateNfts"
    }
    fn agent() -> &'static str {
        "product-factory"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryProductsGenerateNftsReturns(pub bool);
impl Schema for ProductFactoryProductsGenerateNftsReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for ProductFactoryProductsGenerateNftsReturns {
    fn topic() -> &'static str {
        "product-factory_products_generateNfts"
    }
    fn method() -> &'static str {
        "products_generateNfts"
    }
    fn agent() -> &'static str {
        "product-factory"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
