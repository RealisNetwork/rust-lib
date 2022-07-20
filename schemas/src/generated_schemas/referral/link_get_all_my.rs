// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for ReferralLinkGetAllMyParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(ReferralLinkGetAllMyParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct ReferralLinkGetAllMyParams;
impl Schema for ReferralLinkGetAllMyParams {
    fn schema() -> Value {
        serde_json::json!("{}")
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
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralLinkGetAllMyReturnsReferralLinksParamsParams {
    #[serde(rename = "appId")]
    pub app_id: i64,
    #[serde(rename = "link")]
    pub link: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralLinkGetAllMyReturns {
    #[serde(rename = "referralLinks")]
    pub referral_links: Vec<ReferralLinkGetAllMyReturnsReferralLinksParamsParams>,
    #[serde(rename = "referralCode")]
    pub referral_code: String,
}
impl Schema for ReferralLinkGetAllMyReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"referralLinks\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"link\":{\"type\":\"string\"}},\"required\":[\"appId\",\"link\"]}},\"referralCode\":{\"type\":\"string\"}},\"required\":[\"referralCode\",\"referralLinks\"]}")
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
}
