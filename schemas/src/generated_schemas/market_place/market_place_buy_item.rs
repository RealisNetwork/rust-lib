// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceBuyItemParams {
    #[serde(rename = "productId")]
    pub product_id: i32,
    #[serde(rename = "userId")]
    pub user_id: String,
}
pub type MarketPlaceMarketPlaceBuyItemReturns = ();
