// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for ReferralLinkGetAllMyParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(ReferralLinkGetAllMyParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct ReferralLinkGetAllMyParams;
impl Schema for ReferralLinkGetAllMyParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for ReferralLinkGetAllMyParams {
    fn topic() -> &'static str {
        "referral_link_getAllMy"
    }
    fn method() -> &'static str {
        "link_getAllMy"
    }
    fn agent() -> &'static str {
        "referral"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralLinkGetAllMyReturnsReferralLinksParamsParams {
    #[serde(rename = "link", deserialize_with = "deserialize_to_string")]
    pub link: String,
    #[serde(rename = "appId")]
    pub app_id: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralLinkGetAllMyReturns {
    #[serde(rename = "referralCode", deserialize_with = "deserialize_to_string")]
    pub referral_code: String,
    #[serde(rename = "referralLinks")]
    pub referral_links: Vec<ReferralLinkGetAllMyReturnsReferralLinksParamsParams>,
}
impl Schema for ReferralLinkGetAllMyReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"referralCode\":{\"type\":\"string\"},\"referralLinks\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"link\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"appId\",\"link\"]}}},\"required\":[\"referralCode\",\"referralLinks\"]}")
    }
}
impl Agent for ReferralLinkGetAllMyReturns {
    fn topic() -> &'static str {
        "referral_link_getAllMy"
    }
    fn method() -> &'static str {
        "link_getAllMy"
    }
    fn agent() -> &'static str {
        "referral"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
