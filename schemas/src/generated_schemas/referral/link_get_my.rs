// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralLinkGetMyParams {
    #[serde(rename = "appId")]
    pub app_id: i64,
}
impl Schema for ReferralLinkGetMyParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"appId\"]}")
    }
}
impl Agent for ReferralLinkGetMyParams {
    fn topic() -> &'static str {
        "referral_link_getMy"
    }
    fn method() -> &'static str {
        "link_getMy"
    }
    fn agent() -> &'static str {
        "referral"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralLinkGetMyReturns(pub String);
impl Schema for ReferralLinkGetMyReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"string\"}")
    }
}
impl Agent for ReferralLinkGetMyReturns {
    fn topic() -> &'static str {
        "referral_link_getMy"
    }
    fn method() -> &'static str {
        "link_getMy"
    }
    fn agent() -> &'static str {
        "referral"
    }
}
