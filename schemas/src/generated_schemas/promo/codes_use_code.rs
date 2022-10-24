// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoCodesUseCodeParams {
    #[serde(rename = "code")]
    pub code: String,
}
impl Schema for PromoCodesUseCodeParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"code\":{\"type\":\"string\"}},\"required\":[\"code\"]}") . unwrap ()
    }
}
impl Agent for PromoCodesUseCodeParams {
    fn topic() -> &'static str {
        "promo_codes_useCode"
    }
    fn method() -> &'static str {
        "codes_useCode"
    }
    fn agent() -> &'static str {
        "promo"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoCodesUseCodeReturnsParams {
    #[serde(rename = "heroId")]
    pub hero_id: i8,
    #[serde(rename = "currencyAmount")]
    pub currency_amount: String,
    #[serde(rename = "currencyKey")]
    pub currency_key: String,
    #[serde(rename = "rewardType")]
    pub reward_type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoCodesUseCodeReturns(pub Vec<PromoCodesUseCodeReturnsParams>);
impl Schema for PromoCodesUseCodeReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"heroId\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"currencyAmount\":{\"type\":\"string\"},\"currencyKey\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"},\"rewardType\":{\"type\":\"string\"}},\"required\":[\"rewardType\",\"currencyKey\",\"currencyAmount\",\"heroId\"]}}")
    }
}
impl Agent for PromoCodesUseCodeReturns {
    fn topic() -> &'static str {
        "promo_codes_useCode"
    }
    fn method() -> &'static str {
        "codes_useCode"
    }
    fn agent() -> &'static str {
        "promo"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
