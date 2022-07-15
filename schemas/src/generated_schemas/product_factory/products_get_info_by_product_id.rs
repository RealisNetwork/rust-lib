// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductFactoryProductsGetInfoByProductIdParams {
    #[serde(rename = "productId")]
    pub product_id: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductFactoryProductsGetInfoByProductIdReturns {
    #[serde(rename = "subType")]
    pub sub_type: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "isNft")]
    pub is_nft: bool,
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "productType")]
    pub product_type: String,
}
