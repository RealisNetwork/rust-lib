// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralReferralGetReferralInfoListParams {
    #[serde(rename = "perPage")]
    pub per_page: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "page")]
    pub page: f64,
}
impl Schema for ReferralReferralGetReferralInfoListParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"},\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"page\",\"perPage\"]}")
    }
}
impl Agent for ReferralReferralGetReferralInfoListParams {
    fn topic() -> &'static str {
        "referral_referral_getReferralInfoList"
    }
    fn method() -> &'static str {
        "referral_getReferralInfoList"
    }
    fn agent() -> &'static str {
        "referral"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralReferralGetReferralInfoListReturnsDataParamsParams {
    #[serde(rename = "registryDate")]
    pub registry_date: String,
    #[serde(rename = "isBanned")]
    pub is_banned: bool,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "nickname")]
    pub nickname: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralReferralGetReferralInfoListReturns {
    #[serde(rename = "data")]
    pub data: Vec<ReferralReferralGetReferralInfoListReturnsDataParamsParams>,
    #[serde(rename = "totalCount")]
    pub total_count: f64,
}
impl Schema for ReferralReferralGetReferralInfoListReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"data\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"registryDate\":{\"type\":\"string\"},\"isBanned\":{\"type\":\"boolean\"},\"userId\":{\"type\":\"string\"},\"nickname\":{\"type\":\"string\"}},\"required\":[\"userId\",\"nickname\",\"isBanned\",\"registryDate\"]}},\"totalCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"totalCount\",\"data\"]}")
    }
}
impl Agent for ReferralReferralGetReferralInfoListReturns {
    fn topic() -> &'static str {
        "referral_referral_getReferralInfoList"
    }
    fn method() -> &'static str {
        "referral_getReferralInfoList"
    }
    fn agent() -> &'static str {
        "referral"
    }
}
