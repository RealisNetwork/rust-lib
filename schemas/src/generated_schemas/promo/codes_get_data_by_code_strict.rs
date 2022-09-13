// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoCodesGetDataByCodeStrictParams {
    #[serde(rename = "code")]
    pub code: String,
}
impl Schema for PromoCodesGetDataByCodeStrictParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"code\":{\"type\":\"string\"}},\"required\":[\"code\"]}")
    }
}
impl Agent for PromoCodesGetDataByCodeStrictParams {
    fn topic() -> &'static str {
        "promo_codes_getDataByCodeStrict"
    }
    fn method() -> &'static str {
        "codes_getDataByCodeStrict"
    }
    fn agent() -> &'static str {
        "promo"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoCodesGetDataByCodeStrictReturns {}
impl Schema for PromoCodesGetDataByCodeStrictReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"object\",\"properties\":{},\"required\":null}")
    }
}
impl Agent for PromoCodesGetDataByCodeStrictReturns {
    fn topic() -> &'static str {
        "promo_codes_getDataByCodeStrict"
    }
    fn method() -> &'static str {
        "codes_getDataByCodeStrict"
    }
    fn agent() -> &'static str {
        "promo"
    }
}
