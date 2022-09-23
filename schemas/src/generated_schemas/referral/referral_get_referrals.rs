// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralReferralGetReferralsParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "appId")]
    pub app_id: f64,
}
impl Schema for ReferralReferralGetReferralsParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"appId\"]}")
    }
}
impl Agent for ReferralReferralGetReferralsParams {
    fn topic() -> &'static str {
        "referral_referral_getReferrals"
    }
    fn method() -> &'static str {
        "referral_getReferrals"
    }
    fn agent() -> &'static str {
        "referral"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralReferralGetReferralsReturnsReferralTransactionsParamsParams {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "amount")]
    pub amount: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralReferralGetReferralsReturnsReferralsParamsParams {
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "appId")]
    pub app_id: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralReferralGetReferralsReturns {
    #[serde(rename = "referralTransactions")]
    pub referral_transactions:
        Vec<ReferralReferralGetReferralsReturnsReferralTransactionsParamsParams>,
    #[serde(rename = "referrals")]
    pub referrals: Vec<ReferralReferralGetReferralsReturnsReferralsParamsParams>,
}
impl Schema for ReferralReferralGetReferralsReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"referralTransactions\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"date\":{\"type\":\"string\"},\"nickname\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"}},\"required\":[\"date\",\"nickname\",\"amount\"]}},\"referrals\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"nickname\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"nickname\",\"appId\"]}}},\"required\":[\"referrals\",\"referralTransactions\"]}")
    }
}
impl Agent for ReferralReferralGetReferralsReturns {
    fn topic() -> &'static str {
        "referral_referral_getReferrals"
    }
    fn method() -> &'static str {
        "referral_getReferrals"
    }
    fn agent() -> &'static str {
        "referral"
    }
}
