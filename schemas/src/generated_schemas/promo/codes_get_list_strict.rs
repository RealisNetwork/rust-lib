// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for PromoCodesGetListStrictParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(PromoCodesGetListStrictParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct PromoCodesGetListStrictParams;
impl Schema for PromoCodesGetListStrictParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for PromoCodesGetListStrictParams {
    fn topic() -> &'static str {
        "promo_codes_getListStrict"
    }
    fn method() -> &'static str {
        "codes_getListStrict"
    }
    fn agent() -> &'static str {
        "promo"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoCodesGetListStrictReturnsParams(pub Value);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoCodesGetListStrictReturns(pub Vec<PromoCodesGetListStrictReturnsParams>);
impl Schema for PromoCodesGetListStrictReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{}}}")
    }
}
impl Agent for PromoCodesGetListStrictReturns {
    fn topic() -> &'static str {
        "promo_codes_getListStrict"
    }
    fn method() -> &'static str {
        "codes_getListStrict"
    }
    fn agent() -> &'static str {
        "promo"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
