// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralLinkGetParams {
    #[serde(rename = "appId")]
    pub app_id: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for ReferralLinkGetParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"appId\",\"userId\"]}")
    }
}
impl Agent for ReferralLinkGetParams {
    fn topic() -> &'static str {
        "referral_link_get"
    }
    fn method() -> &'static str {
        "link_get"
    }
    fn agent() -> &'static str {
        "referral"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralLinkGetReturns(pub String);
impl Schema for ReferralLinkGetReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"string\"}")
    }
}
impl Agent for ReferralLinkGetReturns {
    fn topic() -> &'static str {
        "referral_link_get"
    }
    fn method() -> &'static str {
        "link_get"
    }
    fn agent() -> &'static str {
        "referral"
    }
}
