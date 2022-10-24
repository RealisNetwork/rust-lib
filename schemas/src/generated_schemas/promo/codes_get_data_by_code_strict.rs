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
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"code\":{\"type\":\"string\"}},\"required\":[\"code\"]}") . unwrap ()
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoCodesGetDataByCodeStrictReturns(Value);
impl Schema for PromoCodesGetDataByCodeStrictReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"object\",\"properties\":{}}")
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
