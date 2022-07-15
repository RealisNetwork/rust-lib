// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct PromoCodesUseCodeParams {
    #[serde(rename = "code")]
    pub code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PromoCodesUseCodeReturnsParams {
    #[serde(rename = "currencyKey")]
    pub currency_key: String,
    #[serde(rename = "heroId")]
    pub hero_id: i8,
    #[serde(rename = "rewardType")]
    pub reward_type: String,
    #[serde(rename = "currencyAmount")]
    pub currency_amount: String,
}
pub type PromoCodesUseCodeReturns = Vec<PromoCodesUseCodeReturnsParams>;
