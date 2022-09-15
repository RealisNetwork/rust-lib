// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralLinkGetAllParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for ReferralLinkGetAllParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
impl Agent for ReferralLinkGetAllParams {
    fn topic() -> &'static str {
        "referral_link_getAll"
    }
    fn method() -> &'static str {
        "link_getAll"
    }
    fn agent() -> &'static str {
        "referral"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralLinkGetAllReturnsReferralLinksParamsParams {
    #[serde(rename = "appId")]
    pub app_id: f64,
    #[serde(rename = "link")]
    pub link: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralLinkGetAllReturns {
    #[serde(rename = "referralLinks")]
    pub referral_links: Vec<ReferralLinkGetAllReturnsReferralLinksParamsParams>,
    #[serde(rename = "referralCode")]
    pub referral_code: String,
}
impl Schema for ReferralLinkGetAllReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"referralLinks\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"link\":{\"type\":\"string\"}},\"required\":[\"appId\",\"link\"]}},\"referralCode\":{\"type\":\"string\"}},\"required\":[\"referralCode\",\"referralLinks\"]}")
    }
}
impl Agent for ReferralLinkGetAllReturns {
    fn topic() -> &'static str {
        "referral_link_getAll"
    }
    fn method() -> &'static str {
        "link_getAll"
    }
    fn agent() -> &'static str {
        "referral"
    }
}
