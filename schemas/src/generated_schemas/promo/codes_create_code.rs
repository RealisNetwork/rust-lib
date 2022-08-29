// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoCodesCreateCodeParamsGivesCurrenciesParams {
    #[serde(rename = "ETH")]
    pub eth: f64,
    #[serde(rename = "LIS")]
    pub lis: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoCodesCreateCodeParamsGivesItemsParams {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoCodesCreateCodeParams {
    #[serde(rename = "numberOfAttempts")]
    pub number_of_attempts: f64,
    #[serde(rename = "givesCurrencies")]
    pub gives_currencies: PromoCodesCreateCodeParamsGivesCurrenciesParams,
    #[serde(rename = "expiresIn")]
    pub expires_in: String,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "givesItems")]
    pub gives_items: PromoCodesCreateCodeParamsGivesItemsParams,
}
impl Schema for PromoCodesCreateCodeParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"numberOfAttempts\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"givesCurrencies\":{\"type\":\"object\",\"properties\":{\"ETH\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"LIS\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"ETH\",\"LIS\"]},\"expiresIn\":{\"type\":\"string\"},\"code\":{\"type\":\"string\"},\"givesItems\":{\"type\":\"object\",\"properties\":{},\"required\":null}},\"required\":[\"code\",\"numberOfAttempts\",\"expiresIn\",\"givesCurrencies\",\"givesItems\"]}")
    }
}
impl Agent for PromoCodesCreateCodeParams {
    fn topic() -> &'static str {
        "promo_codes_createCode"
    }
    fn method() -> &'static str {
        "codes_createCode"
    }
    fn agent() -> &'static str {
        "promo"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoCodesCreateCodeReturns(pub bool);
impl Schema for PromoCodesCreateCodeReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for PromoCodesCreateCodeReturns {
    fn topic() -> &'static str {
        "promo_codes_createCode"
    }
    fn method() -> &'static str {
        "codes_createCode"
    }
    fn agent() -> &'static str {
        "promo"
    }
}
