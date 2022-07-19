// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralReferralGetUserDataParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "appId")]
    pub app_id: i64,
}
impl Schema for ReferralReferralGetUserDataParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"appId\",\"userId\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralReferralGetUserDataReturns {
    #[serde(rename = "refCode")]
    pub ref_code: String,
    #[serde(rename = "refLink")]
    pub ref_link: String,
    #[serde(rename = "hasReferrer")]
    pub has_referrer: bool,
}
impl Schema for ReferralReferralGetUserDataReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"refCode\":{\"type\":\"string\"},\"refLink\":{\"type\":\"string\"},\"hasReferrer\":{\"type\":\"boolean\"}},\"required\":[\"refLink\",\"refCode\",\"hasReferrer\"]}")
    }
}